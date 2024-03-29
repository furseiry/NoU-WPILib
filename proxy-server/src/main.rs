use std::time::{Instant, Duration};
use std::env;
use std::error::Error;
use std::io;
use std::net::{TcpListener, TcpStream};
use std::pin::Pin;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

use simplersble::{Adapter, Peripheral};

use tungstenite::Error::AlreadyClosed;
use tungstenite::Message::Text;

use crate::parsing::*;

mod parsing;

fn get_robot_name() -> String {
    let mut args = env::args();
    args.nth(1).expect("Enter a robot name")
}

fn get_robot_address() -> Result<Pin<Box<Peripheral>>, simplersble::Error> {
    let robot_name = get_robot_name();
    let (scan_sender, scan_receiver) = mpsc::sync_channel(1);
    let mut adapter = Adapter::get_adapters()?.swap_remove(0);
    adapter.set_callback_on_scan_updated(Box::new(move |peripheral| {
        if peripheral.identifier().unwrap() == get_robot_name() {
            scan_sender.send(()).unwrap();
        }
    }));

    adapter.scan_start()?;
    scan_receiver.recv().unwrap();
    adapter.scan_stop()?;
    let results = adapter.scan_get_results()?;

    for peripheral in results {
        if peripheral.identifier()? == robot_name {
            return Ok(peripheral);
        }
    }

    unreachable!()
}

fn start_bluetooth(
    sender_to_sim: &Sender<Option<String>>,
    receiver_from_sim: &Receiver<Option<Vec<u8>>>,
) -> Result<(), simplersble::Error> {
    println!("Searching for robot bluetooth.");

    let mut robot_bt = get_robot_address()?;

    println!("\nFound robot: {}", robot_bt.identifier()?);
    println!("Connecting to robot.");

    robot_bt.connect()?;

    let services = robot_bt.services()?;
    let (mut write_service, mut write_characteristic) = (String::new(), String::new());
    let (mut read_service, mut read_characteristic) = (String::new(), String::new());

    for service in services {
        for characteristic in service.characteristics() {
            if characteristic.can_write_request() {
                write_service = service.uuid();
                write_characteristic = characteristic.uuid();
            }
            if characteristic.can_notify() {
                read_service = service.uuid();
                read_characteristic = characteristic.uuid();
            }
        }
    }

    println!("Connected to robot.");

    let (notify_sender, notify_receiver) = mpsc::sync_channel(1);
    robot_bt.notify(
        &read_service,
        &read_characteristic,
        Box::new(move |data| {
            notify_sender.send(parse_robot_to_sim(data)).unwrap();
        }),
    )?;

    loop {
        if let Ok(false) = robot_bt.is_connected() {
            println!("Lost connection to robot.");
            break;
        }

        if let Ok(message) = notify_receiver.try_recv() {
            sender_to_sim.send(message).unwrap();
        }

        if let Ok(Some(message)) = receiver_from_sim.try_recv() {
            if let Err(err) = robot_bt.write_request(&write_service, &write_characteristic, &message) {
                eprintln!("Lost connection to robot: {err}");
                break;
            }
        }
    }

    Ok(())
}

fn listen_for_robot_bt(
    sender_to_sim: Sender<Option<String>>,
    receiver_from_sim: Receiver<Option<Vec<u8>>>,
) -> Result<(), simplersble::Error> {
    loop {
        start_bluetooth(&sender_to_sim, &receiver_from_sim)?;
    }
}

fn start_websocket(
    stream: TcpStream,
    sender_to_bt: &Sender<Option<Vec<u8>>>,
    receiver_from_bt: &Receiver<Option<String>>,
) {
    let stream_clone = stream.try_clone().unwrap();

    let mut robot_sim_ws = tungstenite::accept(stream).unwrap();

    stream_clone.set_nonblocking(true).unwrap();

    println!("Connected to robot simulator.");

    let mut timer = Instant::now();

    loop {
        if !robot_sim_ws.can_read() || !robot_sim_ws.can_write() {
            eprintln!("Connection to robot simulator lost.");
            break;
        }

        if let Ok(Text(message)) = robot_sim_ws.read_message() {
            parse_sim_to_robot(message);
            if timer.elapsed() > Duration::from_millis(150) {
                timer = Instant::now();
                let builder_ref = PacketBuilder::get_builder_ref();
                let packet_builder = builder_ref.as_ref();
                sender_to_bt.send(packet_builder.build_message()).unwrap();
            }
        }

        if let Ok(Some(message)) = receiver_from_bt.try_recv() {
            if let Err(AlreadyClosed) = robot_sim_ws.write_message(Text(message)) {
                eprintln!("Connection to robot simulator lost.");
                break;
            }
        }
    }
}

fn listen_for_robot_sim(
    sender_to_bt: Sender<Option<Vec<u8>>>,
    receiver_from_bt: Receiver<Option<String>>,
) -> Result<(), io::Error> {
    loop {
        let listener = TcpListener::bind("127.0.0.1:3300").unwrap();
        listener.set_nonblocking(true)?;
        println!("Waiting for robot simulator.");
        loop {
            for stream in listener.incoming() {
                match stream {
                    Ok(s) => {
                        thread::sleep(Duration::from_millis(50));
                        start_websocket(s, &sender_to_bt, &receiver_from_bt);
                    },
                    _ => ()
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Starting NoU proxy, now in rust!\n");

    let (sender_to_sim, receiver_from_bt) = mpsc::channel();
    let (sender_to_bt, receiver_from_sim) = mpsc::channel();

    let bt_listener_thread =
        thread::spawn(|| listen_for_robot_bt(sender_to_sim, receiver_from_sim));
    let sim_listener_thread =
        thread::spawn(|| listen_for_robot_sim(sender_to_bt, receiver_from_bt));

    bt_listener_thread.join().unwrap()?;
    sim_listener_thread.join().unwrap()?;

    Ok(())
}

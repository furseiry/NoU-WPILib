use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use serde_json::{json, Value};

fn read_field_f64(data: &Value, field: &str) -> Option<f64> {
    let wo = data["<".to_string() + field].as_f64();
    let rw = data["<>".to_string() + field].as_f64();
    wo.or(rw)
}

fn read_field_u64(data: &Value, field: &str) -> Option<u64> {
    let wo = data["<".to_string() + field].as_u64();
    let rw = data["<>".to_string() + field].as_u64();
    wo.or(rw)
}

#[derive(Eq, PartialEq, Hash, Clone)]
#[derive(Debug)]
pub enum Device {
    Motor(i8),
    Servo(i8),
    GPIO(i8)
}

#[derive(Clone)]
pub struct PacketBuilder {
    updated_devices: RefCell<HashMap<Device, i8>>
}

thread_local! {
    static BUILDER: Rc<PacketBuilder> = Rc::new(PacketBuilder {
        updated_devices: RefCell::new(HashMap::new())
    })
}

impl PacketBuilder {
    pub fn get_builder_ref() -> Rc<Self> {
        BUILDER.with(|f| f.clone())
    }
    pub fn update(&self, device: Device, value: i8) {
        self.updated_devices.borrow_mut().insert(device, value);
    }
    pub fn build_message(&self) -> Option<Vec<u8>> {
        if self.updated_devices.borrow().is_empty() {
            return None;
        }
        let mut res = String::new();
        for (key, value) in self.updated_devices.borrow_mut().iter() {
            res += &match key {
                Device::Motor(port) => "m".to_string() + "0" + &port.to_string(),
                Device::Servo(port) => "s".to_string() + "0" + &port.to_string(),
                Device::GPIO(port) => "g".to_string() + { if *port >= 10 {""} else {"0"} } + &port.to_string()
            };
            res += &value.to_string();
            res += "\n";
            println!("{key:?}, {value}");
        }
        res += "\0";
        self.updated_devices.borrow_mut().clear();
        println!("{res}\n{:?}", res.as_bytes().to_vec());
        Some(res.as_bytes().to_vec())
    }
}

pub fn parse_sim_to_robot(data: String) -> () {
    let json_data: Value = serde_json::from_str(&data).unwrap();
    if let Some("SimDevice") = json_data["type"].as_str() {
        let (device, mut num) = json_data["device"]
            .as_str()
            .unwrap()
            .split_once("[")
            .unwrap();
        num = num.trim_end_matches("]");
        let num = num.parse().unwrap();

        let builder_ref = PacketBuilder::get_builder_ref();
        let packet_builder = builder_ref.as_ref();
        match device {
            "NoUMotor" => {
                let speed = read_field_f64(&json_data["data"], "speed").unwrap();
                packet_builder.update(Device::Motor(num), (speed * 100.) as i8);
            }
            "NoUServo" => {
                let angle = read_field_f64(&json_data["data"], "angle").unwrap();
                packet_builder.update(Device::Servo(num), (angle) as i8);
            }
            "NoUGPIO" => {
                let value = read_field_u64(&json_data["data"], "value").unwrap_or_default();
                packet_builder.update(Device::GPIO(num), value as i8);
            }
            _ => ()
        }
    }
}

pub fn parse_robot_to_sim(data: Vec<u8>) -> Option<String> {
    let mut result = json!({
        "data": {},
        "device": "",
        "type": "SimDevice"
    });

    let device_num = String::from_utf8_lossy(&data[3..]);

    match data[0] as char {
        'm' => unimplemented!(),
        's' => unimplemented!(),
        'g' => {
            *result.get_mut("device").unwrap() = json!(format!("NoUGPIO[{device_num}]"));
            let key = match data[1] as char {
                '0' => ">value".to_string(),
                '2' => "<>value".to_string(),
                _ => unreachable!(),
            };
            result["data"]
                .as_object_mut()
                .unwrap()
                .insert(key, json!(data[2] - '0' as u8));
        }
        'p' => unimplemented!(),
        _ => (),
    }
    serde_json::to_string(&result).ok()
}

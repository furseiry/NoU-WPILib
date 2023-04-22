const { WebSocketServer } = require('ws');

const wss = new WebSocketServer({ port: 3300 });
const btSerial = new (require('node-bluetooth-serial-port').BluetoothSerialPort)();

const ROBOT_NAME = "";

let robotBT;
let robotSim;

startProxy();

function startProxy() {
  console.log('Starting NoU2 Proxy.');

  // List all connected bluetooth devices and check if one of them matches robot name.
  btSerial.listPairedDevices(list => {
    robotBT = list.find(device => device.name == ROBOT_NAME);
    if (robotBT) {
      foundRobot();
    }
  });
}

function foundRobot() {
  console.log('Found Robot.');

  // Wait for connection to robot simulator.
  wss.on('connection', simulatorConnected);
}

function simulatorConnected(ws) {
  console.log('Connected to robot simulator.');
  robotSim = ws;

  // If bluetooth connection is already open, don't attempt to connect again.
  if (btSerial.isOpen()) {
    robotConnected();
    return;
  }

  btSerial.connect(robotBT.address, robotBT.services[0].channel, robotConnected);
}

function robotConnected() {
  console.log('Connected to robot.');

  // Set up message passing.
  robotSim.on('message', handleSimToRobot);
  // btSerial.on('data', handleRobotToSim);
}

function handleSimToRobot(data) {
  const jsonData = JSON.parse(data);

  if (jsonData.type != 'SimDevice') return;

  let message = '';
  let deviceNum = jsonData.device.match(/\[(\d+)]$/)[1];

  switch (jsonData.device.replace(/\[\d+]$/, "")) {
    case 'NoUMotor':
      message += 'm';
      message += deviceNum;
      message += Math.round(100 * jsonData.data['<speed']) / 100;
      break;
    case 'NoUServo':
      message += 's';
      message += deviceNum;
      message += Math.round(100 * jsonData.data['<angle']) / 100;
      break;
    case 'NoUGPIO':
      message += 'g';
      message += jsonData.data['<>value'] || jsonData.data['<value'] || 0;
      message += deviceNum;
      break;
    case 'GPIOPrep':
      message += 'p';
      message += jsonData.data['<mode'];
      message += deviceNum;
      break;
  }

  btSerial.write(Buffer.from(message + '\0'), () => { });
}

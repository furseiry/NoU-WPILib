const { WebSocketServer } = require('ws');

const wss = new WebSocketServer({ port: 3300 });
const btSerial = new (require('node-bluetooth-serial-port').BluetoothSerialPort)();

const ROBOT_NAME = "";

console.log('Starting NoU2-ws');

btSerial.listPairedDevices(list => {
  const robot = list.find(device => device.name == ROBOT_NAME);

  if (!robot) return;

  console.log('Found robot')

  wss.on('connection', ws => {
    console.log('Connected to robot simulator');

    btSerial.connect(robot.address, robot.services[0].channel, () => {
      console.log('Connected to robot');

      ws.on('message', data => {
        const jsonData = JSON.parse(data);

        if (jsonData.type != 'PWM') return;
        if (jsonData.device > 6 && jsonData.device < 11 || jsonData.device < 1 || jsonData.device > 14) return;

        const type = jsonData.device < 7 ? 'm' : 's';
        const output = Math.round(jsonData.data['<speed'] * 100) / 100;

        btSerial.write(Buffer.from(type + jsonData.device + output + '\0'), () => { });
      });
    });
  });
});

#include <BluetoothSerial.h>
#include <Alfredo_NoU2.h>

BluetoothSerial btSerial;

String ROBOT_NAME = "";

NoU_Motor motors[6] = {
    NoU_Motor(1),
    NoU_Motor(2),
    NoU_Motor(3),
    NoU_Motor(4),
    NoU_Motor(5),
    NoU_Motor(6),
};
NoU_Servo servos[4] = {
    NoU_Servo(1),
    NoU_Servo(2),
    NoU_Servo(3),
    NoU_Servo(4),
};

void setup()
{
  btSerial.begin(ROBOT_NAME);
}

String currentMessage = "";

void loop()
{
  if (!btSerial.available())
    return;

  char c = btSerial.read();
  currentMessage += c;

  if (c == '\0')
  {    
    char type = currentMessage[0];

    switch (type)
    {
    case 'm':
    {
      float speed = currentMessage.substring(2).toFloat();
      int deviceNum = currentMessage[1] - '1';
      motors[deviceNum].set(speed);
      break;
    }
    case 's':
    {
      float angle = currentMessage.substring(2).toFloat();
      int deviceNum = currentMessage[1] - '1';
      servos[deviceNum].write(angle);
      break;
    }
    case 'g':
    {
      int value = currentMessage.charAt(1) - '0';
      int deviceNum = currentMessage.substring(2).toInt();
      digitalWrite(deviceNum, value);
      break;
    }
    case 'p':
    {
      int mode = currentMessage.charAt(1) - '0';
      int deviceNum = currentMessage.substring(2).toInt();
      pinMode(deviceNum, mode);
      break;
    }
    }

    currentMessage = "";
  }
}

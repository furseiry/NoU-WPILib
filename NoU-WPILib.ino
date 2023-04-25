#include <BleSerial.h>
#include <Alfredo_NoU2.h>

BleSerial btSerial;

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

/* typedef struct {
  int mode;
  int pin;
} pin_config_t;

void sendPinData(void* pinConfig) {
  pin_config_t* config = (pin_config_t*) pinConfig;
  while(1) {
    btSerial.printf("g%d%d%d", 0, digitalRead(2), 1);
    vTaskDelay(10);
  }
} */

void setup()
{
  btSerial.begin(ROBOT_NAME.c_str());
}

void loop()
{
  while (btSerial.available())
  {
    String currentMessage = btSerial.readStringUntil('\0');

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
      /* if (mode == 0 || mode == 2) {
        pin_config_t config = {
          .mode = mode,
          .pin = deviceNum
        };
        xTaskCreate(sendPinData, "sendPinData", 32768, &config, 1, NULL);
      } */
      break;
    }
    }
  }
}
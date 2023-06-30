#include <BleSerial.h>
#include <Alfredo_NoU2.h>
#include <esp_log.h>

BleSerial btSerial;

String ROBOT_NAME = "twink";

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
  Serial.begin(115200);
  btSerial.begin(ROBOT_NAME.c_str(), true, 2);
  Serial.println(ROBOT_NAME);
}

void loop()
{
  while (btSerial.available())
  {
    String packet = btSerial.readStringUntil('\0');
    Serial.println(packet);

    int lastIndex = 0;
    int deviceIndex = packet.indexOf('\n', lastIndex);
    Serial.println(deviceIndex);
    while (deviceIndex != -1) {
      String deviceMessage = packet.substring(lastIndex, deviceIndex);
      Serial.println("\t" + deviceMessage);

      lastIndex = deviceIndex + 1;
      deviceIndex = packet.indexOf('\n', lastIndex);
    Serial.println(deviceIndex);


      char type = deviceMessage[0];

      int port = deviceMessage.substring(1,3).toInt();

      int value = deviceMessage.substring(3).toInt();

      Serial.printf("Type:%c, Port:%i, Value:%i\n", type, port, value);

      switch (type)
      {
      case 'm':
      {
        float speed = value * 0.01;
        motors[port].set(speed);
        break;
      }
      case 's':
      {
        float angle = value * 0.0055;
        servos[port].write(angle);
        break;
      }
      case 'g':
      {
        pinMode(port, 3);
        digitalWrite(port, value);
        break;
      }
      }
    }
  }
}

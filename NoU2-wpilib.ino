#include <BluetoothSerial.h>
#include <ArduinoJson.h>
#include <Alfredo_NoU2.h>

BluetoothSerial btSerial;

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

void setup() {
  Serial.begin(115200);
  btSerial.begin("AraAra");
}

String currentMessage = "";
void loop() {
  if (btSerial.available()) {
    char c = btSerial.read();
    currentMessage += c;
    Serial.println(c);
    if (c == '\0') {
      char type = currentMessage[0];
      if (type == 'm') {
        int motor = currentMessage.substring(1, 2).toInt() - 1;
        float output = currentMessage.substring(2).toFloat();
        motors[motor].set(output);
      } else if (type == 's') {
        int servo = currentMessage.substring(1, 2).toInt() - 1;
        float angle = currentMessage.substring(2).toFloat() * 180;
        servos[servo].write(angle);
      }
      currentMessage = "";
    }
  }
}

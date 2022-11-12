#include <BluetoothSerial.h>
#include <ArduinoJson.h>
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

void setup() {
  btSerial.begin(ROBOT_NAME);
}

// Need to store incoming characters and build each message
String currentMessage = "";

void loop() {
  // If there is data on the serial port
  if (btSerial.available()) {
    // Read one character and store it
    char c = btSerial.read();
    currentMessage += c;
    // Check for message delimiter
    if (c == '\0') {
      // Parse motor type and port
      char type = currentMessage[0];
      int port = currentMessage.substring(1, 2).toInt() - 1;
      if (type == 'm') {
        // Calculate output and set motor
        float output = currentMessage.substring(2).toFloat();
        motors[port].set(output);
      } else if (type == 's') {
        // Calculate angle and set servo
        float angle = currentMessage.substring(2).toFloat() * 180;
        servos[port].write(angle);
      }
      // Reset message for next loop
      currentMessage = "";
    }
  }
}

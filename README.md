# NoU2-wpilib
WPILib integration with NoU2 motor controller using ESP32
## Prerequisites
- NoU2 library
  - Follow the instructions from AlredoSystems [here](https://github.com/AlfredoSystems/Alfredo-NoU2).
    - This should guide you through installing the Arduino IDE if you haven't already, along with configuring it for the ESP32.
- Node.JS
  - This is needed for running the proxy server.
  - Download from their website [here](https://nodejs.org/en/download/). 
- WPILib
  - This is how you're going to be programming your robot.
  - Follow the installation guide [here](https://docs.wpilib.org/en/stable/docs/zero-to-robot/step-2/wpilib-setup.html).
    - The install mode you want is `Everything`.
    - You want to `Download for this computer only` most likely.
## Details
This code has three different parts:
  - The code running on the ESP32 which receives messages from the computer and drives the motors.
  - The proxy server which sends messages to the ESP32 and receives messages from the robot simulator.
  - The robot simulator which runs your WPILib robot code and sends messages to the websocket proxy.
## Setup
You should have all of the prerequisites now. If you do not, go download them [now](https://github.com/afredge/NoU2-wpilib#Prerequisites).

- **Setting up the ESP32**
  - Open `NoU2-wpilib.ino` in the Arduino IDE.
  - Change the name of your robot at the top of the file. This will be the name of the bluetooth device and should not have any spaces.
  - Upload the code to the ESP32. You should have done this before when testing the NoU2 library.
- **Setting up the proxy server**
  - Open `index.js` in a text editor. Change the robot name at the top of the file to the same name you used in the previous step.
  - Open a terminal in the `Proxy server` folder.
  - Run `npm i` to install dependencies.
  - Use `node .` to start the server.
- **Setting up WPILib**
  - Create a new project for your robot. In Visual Studio Code press `Ctrl+Shift+P` then type `WPILib: Create a new project`.
    - You can use an example if you'd like but I prefer the `Command Robot` template.
  - Replace the contents of the generated `build.gradle` file with the one included here.
    - You may need to change the version of the `edu.wpi.first.GradleRIO` plugin at the top to match your version of WPILib.
  - `Ctrl+Shift+P` and `WPILib: Simulate Robot Code` will start the robot simulator.
    - Run with all 3 extensions when prompted.
## Use
1. Connect to robot over bluetooth.
    - You will only have to do this once, or if you change the name of your robot.
2. Start the proxy server.
3. Start robot simulator.

If any part of this system changes, all following parts must be restarted as well. 
## Programming
The NoU2-wpilib software currently only has support for **outputting** to the 6 motor ports and 4 servo ports on the NoU2 motor controller. 
The classes usd to control motors in WPILib are `Spark` and `Servo` respectively. 
The motors are on channels 1-6 and the servos are on channels 11-14.
## Things I'm working on 
- Add GPIO support.
- Custom improved classes for motors and servos.
- Option to connect over wifi instead of bluetooth.

Contact me on discord @choose42#3361 for questions/help.

# NoU-WPILib
WPILib integration with NoU motor controller using ESP32
## Features
- Full support for motors and servos.
- Partial GPIO support.
- Automatic reconnecting for minimum hassle.
## Disclaimer
- I do not know much about Windows so these instructions may not be 100% accurate. I'm working on this.
## Prerequisites
- NoU2 library
  - Follow the instructions from AlfredoSystems [here](https://github.com/AlfredoSystems/Alfredo-NoU2).
    - This should guide you through installing the Arduino IDE if you haven't already, along with configuring it for the ESP32.
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
  - Open `NoU-WPILib.ino` in the Arduino IDE.
  - Change the name of your robot at the top of the file. This will be the name of the bluetooth device and should not have any spaces.
  - Upload the code to the ESP32. You may have done this before when testing the NoU2 library.
- **Setting up the proxy server**
  - Using a precompiled binary:
    - On this GitHub repository navigate to `Actions`
    - Click the most recent one
    - Scroll down to `Artifacts` (scrolling in the main box doesn't work very well, keep your mouse to the left of your screen).
    - Download the artifact corresponding to your operating system.
    - Unzip the downloaded file wherever you like.
    - To start the proxy:
      - Open a terminal in the folder you unzipped `proxy-server` to.
      - Type `./proxy-server` followed by a space and then your robot's name.
        - You may need to run `chmod 755 proxy-server` once before attempting to start the proxy. 
  - Compiling from source:
    - Install [Cargo](https://www.rust-lang.org/tools/install).
    - Open a terminal in the `proxy-server` folder.
    - Run `cargo run` followed by a space and then your robot's name.

- **Setting up WPILib**
  - Create a new project for your robot. In Visual Studio Code press `Ctrl+Shift+P` then type `WPILib: Create a new project`.
    - Currently the library only has java support.
    - Whether you use an example or a template doesn't really matter. Just stick with the ones designed for regular robots.
  - Add the following code to your build.gradle: 
```
wpi.sim.addGui().defaultEnabled = true
wpi.sim.addWebsocketsServer().defaultEnabled = true
wpi.sim.addWebsocketsClient().defaultEnabled = true
wpi.sim.envVar("HALSIMWS_HOST", "127.0.0.1")
```
  - Pressing `Ctrl+Shift+P` and selecting `WPILib: Simulate Robot Code` will start the robot simulator.
    - Run with all 3 extensions when prompted.
  - Alternatively you can press `F5`.
## Use
1. Start the proxy server.
2. Start robot simulator.
- Power on the robot at some point

If the proxy server stops the robot simulator must also be restarted. 
## Programming
- In your WPILib project press `Ctrl+Shift+P` and type `WPILib: Manage Vendor Libraries`
- Select `Install new libraries (online)`
- Enter this link: `https://furseiry.github.io/NoULib/NoULib.json`
  - If this doesn't work, try going to the link in your browser, then copy & paste the contents there to a new file called `NoULib.json` in the `vendordeps` folder of your WPILib project.
- It will suggest running a build. Do this so WPILib downloads the library.
  - If you added the file manually, `Ctrl+Shift+P` -> `WPILib: Build Robot Code` will do it.
- Use the provided `NoUMotor`, `NoUServo`, and `NoUGPIO` classes to program your robot! 
## Things I'm working on 
###### (Vaguely in order of priority)
- Better cross platform documentation.
- Read gpio pins correctly
- cpp WPILib classes
- Proxy auto starting
- Option to connect over wifi instead of bluetooth.
- I had a silly idea for integrating the proxy into the vendordep.

Contact me on discord @choose42#7347 for questions/help.

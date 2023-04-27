# NoU-WPILib
WPILib integration with NoU motor controller using ESP32
## Features
- Full support for motors and servos.
- Partial GPIO support (Write only).
- Automatic reconnecting for minimum hassle.
## Prerequisites
- NoU2 library
  - Follow the instructions from AlfredoSystems [here](https://github.com/AlfredoSystems/Alfredo-NoU2).
    - This should guide you through installing the Arduino IDE if you haven't already, along with configuring it for the ESP32.
- WPILib
  - This is how you're going to be programming your robot.
  - Follow the installation guide [here](https://docs.wpilib.org/en/stable/docs/zero-to-robot/step-2/wpilib-setup.html).
    - The install mode you want is `Everything`.
    - You want to `Download for this computer only` unless you are using your own Visual Studio Code (VSCode) installation (in which case you're on your own).
- Familiarity with VSCode is helpful but not required.
## Details
This software has three different parts:
  - **The ESP32 code** which receives instructions from the robot simulator (via the proxy server) to run motors, servos, etc. It also passes information back to the robot simulator.
  - **The proxy server** which runs locally on your computer and is responsible for connecting the ESP32 to the robot simulator and passing messages between them.
  - **The robot simulator** which runs your WPILib robot code and sends messages to the robot (the ESP32) via the proxy server. It also can receive data from the ESP32 code. 
## Setup
You should have all of the prerequisites now. If you do not, go download them [here](https://github.com/afredge/NoU2-wpilib#Prerequisites).

- **Setting up the ESP32**
  - Open `NoU-WPILib.ino` in the Arduino IDE.
  - Name your robot inside the quotes on line 6 (`String ROBOT_NAME = ""`). This will become the name of the bluetooth device so it should not have any spaces.
  - Upload the code to the ESP32. You may have done this before when testing the NoU2 library.
- **Setting up the proxy server**
  - Using a precompiled binary (preferred):
    - On this GitHub repository navigate to `Actions`
    - Click the most recent workflow run (the top one). This will open a page containing details of the run.
    - Scroll down to `Artifacts`.
    - Download the artifact corresponding to your operating system.
    - Unzip the downloaded file wherever you like.
    - To start the proxy server:
      - Open a terminal in the folder containing the unzipped `proxy-server` executable:
        - Open the folder in Finder (Mac) / File Explorer (Windows).
        - Right click the folder path and select `Open in Terminal` (Mac) / `Open in Command Prompt` (Windows).
        - You can alternatively open Terminal (Mac) / Command Prompt (Windows) and navigate using the `cd` command to the folder.
      - On Mac, you will need to run `chmod 755 proxy-server` in order to make the file executable. This only needs to be done once.
      - Type `./proxy-server` (Mac) / `./proxy-server.exe` (Windows) followed by your robot's name (eg. `./proxy-server my-robot`).
  - Compiling from source (try this if the above does not work or is not available for your OS):
    - Install [Cargo](https://www.rust-lang.org/tools/install), a tool for compiling and running rust programs.
    - Open a terminal in the `proxy-server` folder, as described above.
    - Run `cargo run` followed by your robot's name (eg. `cargo run my-robot`).

- **Setting up WPILib**
  - Create a new project for your robot:
    - In VSCode (which will be downloaded by the installer), Open the Command Palette by clicking the little WPILib icon in the top right (shortcut `Cmd+Shift+P` (Mac) / `Ctrl+Shift+P` (Windows))
    - Type `WPILib: Create a new project`. This opens the WPILib Project Creator.
    - Select a project type. Templates work better, but you can try modifying the examples if you want.
    - Select a language (currently this software only has java support)
    - Select a project base:
      - Templates that work the best: Command Robot, Command Robot Skeleton, Timed Robot, Timed Skeleton, RobotBase Skeleton
    - Fill in the rest of the fields and `Generate Project`.
  - Find the build.gradle file in your new WPILib project.
  - Paste the following code to the end of it (this code configures the WPILib robot simulator to send messages to the proxy server):
~~~
try {wpi.sim.addGui().defaultEnabled = true}
catch (Exception _) {}
wpi.sim.addWebsocketsClient().defaultEnabled = true
wpi.sim.envVar("HALSIMWS_HOST", "127.0.0.1")
~~~
  - To start the robot simulator (shortcut `F5`):
    - Open the Command Palette and select `WPILib: Simulate Robot Code`.
    - Run with `Sim GUI` and `Websocket Client` extensions when prompted.
## Programming setup
- In your WPILib project open the Command Palette and type `WPILib: Manage Vendor Libraries`
- Select `Install new libraries (online)`
- Enter this link: `https://furseiry.github.io/NoULib/NoULib.json`
- It will suggest running a build. Do this so WPILib downloads the library.
- If you get an error, add the library manually:
  - Go to the above link in your browser.
  - Copy and paste the contents there to a new file called `NoULib.json` in the `vendordeps` folder of your WPILib project.
  - Open the Command Palette and run `WPILib: Build Robot Code`.
- Use the provided `NoUMotor`, `NoUServo`, and `NoUGPIO` classes to program your robot! 
## Use
1. Power on the robot (ESP32). 
2. Start the proxy server (with your robot name).
3. Start the robot simulator.

The proxy server will automatically reconnect to the ESP32 and the robot simulator if either connection is lost.  
If the proxy server is restarted for any reason, the robot simulator will also need to be restarted.
 
## Things I'm working on 
<sup><sup>(Vaguely in order of priority)
- Better cross platform documentation
- Read gpio pins
- c++ WPILib classes
- Proxy auto starting
- Option to connect over wifi instead of bluetooth.
- I had a silly idea for integrating the proxy into the vendordep.

Contact me on discord @choose42#7347 for questions/help.

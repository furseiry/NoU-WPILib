package frc.robot.NoU2Classes;

import edu.wpi.first.hal.SimDevice;
import edu.wpi.first.hal.SimDevice.Direction;
import edu.wpi.first.hal.SimDouble;
import edu.wpi.first.wpilibj.motorcontrol.MotorController;

public class NoUMotor implements MotorController {
  private SimDevice simDevice;
  private SimDouble speed;
  private boolean isInverted;

  public NoUMotor(int port) {
    if (port < 1 || port > 6) {
      throw new IllegalArgumentException("Port must be in range 1-6");
    }

    simDevice = SimDevice.create("NoUMotor", port);
    speed = simDevice.createDouble("speed", Direction.kOutput, 0.0);
  }

  @Override
  public void set(double speed) {
    if (speed < -1.0 || speed > 1.0) return;
    this.speed.set(isInverted ? -speed : +speed);
  }

  @Override
  public double get() {
    return this.speed.get();
  }

  @Override
  public void setInverted(boolean isInverted) {
    this.isInverted = isInverted;    
  }

  @Override
  public boolean getInverted() {
    return this.isInverted;
  }

  @Override
  public void disable() {
    this.simDevice.close();
    this.simDevice = null;
  }

  @Override
  public void stopMotor() {
    this.set(0.0);
  }
}

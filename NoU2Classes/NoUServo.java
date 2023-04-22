package frc.robot.NoU2Classes;

import edu.wpi.first.hal.SimDevice;
import edu.wpi.first.hal.SimDevice.Direction;
import edu.wpi.first.hal.SimDouble;

public class NoUServo {
  private SimDevice simDevice;
  private SimDouble angle;

  public NoUServo(int port) {
    if (port < 1 || port > 4) {
      throw new IllegalArgumentException("Port must be in range 1-4");
    }

    simDevice = SimDevice.create("NoUServo", port);
    angle = simDevice.createDouble("angle", Direction.kOutput, 0.0);
  }

  public void setAngle(double angle) {
    if (angle < 0.0 || angle > 180.0) return;
    this.angle.set(angle);
  }

  public double getAngle() {
    return this.angle.get();
  }
}

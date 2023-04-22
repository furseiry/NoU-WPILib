package frc.robot.NoU2Classes;

import edu.wpi.first.hal.SimDevice;
import edu.wpi.first.hal.SimDevice.Direction;
import edu.wpi.first.hal.SimInt;

public class NoUGPIO {
  public enum GPIOMode {
    READ_ONLY(Direction.kInput),
    WRITE_ONLY(Direction.kOutput),
    READ_WRITE(Direction.kBidir);

    Direction value;

    private GPIOMode(Direction value) {
      this.value = value;
    }
  }

  private SimDevice simDevice;
  private SimInt value;
  private GPIOMode pinMode;

  public NoUGPIO(int pin, GPIOMode mode) {
    simDevice = SimDevice.create("NoUGPIO", pin);
    value = simDevice.createInt("value", mode.value, 0);
    pinMode = mode;

    var prepDevice = SimDevice.create("GPIOPrep", pin);
    prepDevice.createInt("mode", Direction.kOutput, 0).set(mode.ordinal() + 1);
    prepDevice.close();
  }

  public void set(int value) {
    if (value != 0 && value != 1) {
      throw new IllegalArgumentException("Value must be 0 or 1");
    }
    if (pinMode == GPIOMode.READ_ONLY) {
      throw new UnsupportedOperationException("Attempt to write to GPIO pin in read only mode");
    }

    this.value.set(value);
  }

  public int get() {
    return this.value.get();
  }
}

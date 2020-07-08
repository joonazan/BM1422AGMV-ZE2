#include <FastBLE.h>
#include "reading.h"
#include "magnetometer.h"
#include "field_strength.h"

void setup() {}

void loop() {
  // 0xe if address select is soldered to the right
  // 0xf if address select is soldered to the left
  Magnetometer magnetometer(I2C, 0xe, PIO_SERCOM_ALT);

  FieldStrengthExtractor filter;

  auto field_strength_out = BLE.add_output<FieldStrengthsSquared>(UUID_128(0x2d, 0x71, 0xa2, 0x59, 0xb4, 0x58, 0xc8, 0x12, 0x99, 0x99, 0x43, 0x95, 0x12, 0x2f, 0x46, 0x59));
  BLE.start(UUID_128(0x2a, 0x71, 0xa2, 0x59, 0xb4, 0x58, 0xc8, 0x12, 0x99, 0x99, 0x43, 0x95, 0x12, 0x2f, 0x46, 0x59), "magnetometer", NULL);

  auto last_measurement = millis();

  while (1) {
    FieldStrengthsSquared fs;
    for (int i = 0; i < 20; i++) {
      while (last_measurement == millis());
      last_measurement = millis();
      fs = filter.process(magnetometer.read());

      BLE.poll();
    }

    field_strength_out.write(fs);
  }
}

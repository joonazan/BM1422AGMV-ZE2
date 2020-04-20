#include <FastBLE.h>
#include "reading.h"
#include "magnetometer.h"

void setup() {
  Serial.begin(9600);
  while(!Serial);
}

void loop() {
  // 0xe if address select is soldered to the right
  // 0xf if address select is soldered to the left
  Magnetometer magnetometer(I2C, 0xe, PIO_SERCOM_ALT);

  auto field_strength_out = BLE.add_output<Reading>(UUID_128(0x2d, 0x71, 0xa2, 0x59, 0xb4, 0x58, 0xc8, 0x12, 0x99, 0x99, 0x43, 0x95, 0x12, 0x2f, 0x46, 0x59));
  BLE.start(UUID_128(0x2a, 0x71, 0xa2, 0x59, 0xb4, 0x58, 0xc8, 0x12, 0x99, 0x99, 0x43, 0x95, 0x12, 0x2f, 0x46, 0x59), NULL);

  while (1) {
    auto maxdiff = 0;
    for (int i = 0; i < 1000; i++) {
      auto t1 = micros();

      field_strength_out.write(magnetometer.read());
      delayMicroseconds(400);
      BLE.poll();

      auto t2 = micros();
      auto diff = t2 - t1;
      if (diff > maxdiff) {
        maxdiff = diff;
      }
    }
    Serial.println(maxdiff);
  }
}

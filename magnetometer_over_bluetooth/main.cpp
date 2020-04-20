#include <FastBLE.h>
#include <I2C_DMAC.h>
#include "reading.h"

void initialize_magnetometer(uint8_t slave_address) {
  I2C.begin(400000, REG_ADDR_8BIT, PIO_SERCOM_ALT);

  // continous 14-bit measurement at 1kHz
  I2C.writeByte(slave_address, 0x1b, 0b11011000);

  I2C.writeByte(slave_address, 0x5c, 0x00);
  I2C.writeByte(slave_address, 0x5d, 0x00);

  // start measuring
  I2C.writeByte(slave_address, 0x1d, 0x40);
}

struct Reading get_reading(uint8_t slave_address) {
  Reading r;
  I2C.readBytes(slave_address, 0x10, reinterpret_cast<uint8_t*>(&r), sizeof(Reading));

  return r;
}

// 0xe if address select is soldered to the right
// 0xf if address select is soldered to the left
const uint8_t slave_address = 0xe;

void setup() {
  Serial.begin(9600);
  while(!Serial);

  initialize_magnetometer(slave_address);
}

void loop() {
  auto field_strength_out = BLE.add_output<Reading>(UUID_128(0x2d, 0x71, 0xa2, 0x59, 0xb4, 0x58, 0xc8, 0x12, 0x99, 0x99, 0x43, 0x95, 0x12, 0x2f, 0x46, 0x59));
  BLE.start(UUID_128(0x2a, 0x71, 0xa2, 0x59, 0xb4, 0x58, 0xc8, 0x12, 0x99, 0x99, 0x43, 0x95, 0x12, 0x2f, 0x46, 0x59), NULL);

  while (1) {
    auto maxdiff = 0;
    for (int i = 0; i < 1000; i++) {
      auto t1 = micros();

      field_strength_out.write(get_reading(slave_address));
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

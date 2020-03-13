#include <FastBLE.h>
#include <Wire.h>

void write_register(uint8_t slave_address, uint8_t reg, uint8_t val) {
  Wire.beginTransmission(slave_address);
  Wire.write(reg);
  Wire.write(val);
  Wire.endTransmission();
}

void initialize_magnetometer(uint8_t slave_address) {
  Wire.begin();
  Wire.setClock(400000);

  // continous 14-bit measurement at 1kHz
  write_register(slave_address, 0x1b, 0b11011000);

  write_register(slave_address, 0x5c, 0x00);
  write_register(slave_address, 0x5d, 0x00);

  // turn on DRDY pin
  write_register(slave_address, 0x1c, 0x0c);

  // start measuring
  write_register(slave_address, 0x1d, 0x40);
}

struct FieldStrength {
  int16_t x;
  int16_t y;
  int16_t z;
};

struct FieldStrength get_reading(uint8_t slave_address) {
  Wire.beginTransmission(slave_address);
  Wire.write(byte(0x10));
  Wire.endTransmission();

  Wire.requestFrom(slave_address, 6);
  FieldStrength fs;
  for (int i = 0; i < 6; i++) {
    reinterpret_cast<uint8_t*>(&fs)[i] = Wire.read();
  }

  return fs;
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
  auto field_strength_out = BLE.add_output<FieldStrength>(UUID_128(0x2d, 0x71, 0xa2, 0x59, 0xb4, 0x58, 0xc8, 0x12, 0x99, 0x99, 0x43, 0x95, 0x12, 0x2f, 0x46, 0x59));
  BLE.start(UUID_128(0x2a, 0x71, 0xa2, 0x59, 0xb4, 0x58, 0xc8, 0x12, 0x99, 0x99, 0x43, 0x95, 0x12, 0x2f, 0x46, 0x59), NULL);

  while (1) {
    auto maxdiff = 0;
    for (int i = 0; i < 1000; i++) {
      auto t1 = micros();

      field_strength_out.write(get_reading(slave_address));
      delay(10);
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

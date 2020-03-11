#include <ArduinoBLE.h>
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

BLEService field_strength_service("fba38a6b-443d-406b-b95b-716234425056");
BLETypedCharacteristic<FieldStrength> field_strength_char(
  "c478c8cc-1287-4b01-b503-87399d9d835f",
  BLERead | BLENotify
);

// 0xe if address select is soldered to the right
// 0xf if address select is soldered to the left
const uint8_t slave_address = 0xe;

void setup() {
  Serial.begin(9600);

  initialize_magnetometer(slave_address);
  pinMode(6, INPUT);

  // Start bluetooth

  if (!BLE.begin()) {
    Serial.println("Bluetooth failed.");
    while (1) {}
  }

  // This would minimize latency but it slows down transfer speed too much.
  // It can be enabled if positions are calculated on this machine, as that
  // reduces the bandwidth needed.
  //BLE.setConnectionInterval(0x0006, 0x0006);

  field_strength_service.addCharacteristic(field_strength_char);
  BLE.addService(field_strength_service);

  FieldStrength fs = {0};
  field_strength_char.writeValue(fs);

  BLE.setLocalName("position sensor");
  BLE.setAdvertisedService(field_strength_service);
  BLE.advertise();
}

void loop() {
  auto maxdiff = 0;
  for (int i = 0; i < 1000; i++) {
    auto t1 = micros();
    FieldStrength fs = {x: 1, y: 2, z: i};
    field_strength_char.writeValue(fs);
    BLE.poll();
    auto t2 = micros();
    auto diff = t2 - t1;
    if (diff > maxdiff) {
      maxdiff = diff;
    }
  }
  Serial.println(maxdiff);
}

#include <ArduinoBLE.h>

struct FieldStrength {
  int16_t x;
  int16_t y;
  int16_t z;
};

BLEService field_strength_service("fba38a6b-443d-406b-b95b-716234425056");
BLETypedCharacteristic<FieldStrength> field_strength_char(
  "c478c8cc-1287-4b01-b503-87399d9d835f",
  BLERead | BLENotify
);

uint64_t last_time = 0;

void setup() {
  Serial.begin(9600);

  if (!BLE.begin()) {
    Serial.println("Bluetooth failed.");
    while(1){}
  }

  field_strength_service.addCharacteristic(field_strength_char);
  BLE.addService(field_strength_service);

  FieldStrength fs = {0};
  field_strength_char.writeValue(fs);

  BLE.setLocalName("position sensor");
  BLE.setAdvertisedService(field_strength_service);
  BLE.advertise();
}

void loop() {
  BLE.poll();

  uint64_t time = millis();
  if (time > last_time) {
    FieldStrength fs = {x: 1, y: -2, z: 700};
    field_strength_char.writeValue(fs);
    last_time = time;
  }
}

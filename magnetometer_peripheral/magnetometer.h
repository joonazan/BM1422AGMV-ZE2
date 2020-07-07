#pragma once

#include <I2C_DMAC.h>
#include "reading.h"

class Magnetometer {
  I2C_DMAC& i2c;
  const uint8_t slave_address;
public:
  Magnetometer(I2C_DMAC& device, uint8_t address, EPioType mode) :
    i2c(device),
    slave_address(address) {
    i2c.begin(400000, REG_ADDR_8BIT, mode);

    // continous 14-bit measurement at 1kHz
    i2c.writeByte(slave_address, 0x1b, 0b11011000);

    i2c.writeByte(slave_address, 0x5c, 0x00);
    i2c.writeByte(slave_address, 0x5d, 0x00);

    // start measuring
    i2c.writeByte(slave_address, 0x1d, 0x40); 
  }

  Reading read() {
    Reading r;
    i2c.readBytes(slave_address, 0x10, reinterpret_cast<uint8_t*>(&r), sizeof(Reading));

    return r;
  }
};

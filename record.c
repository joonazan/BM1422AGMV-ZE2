#include <stdio.h>
#include <stdint.h>
#include <unistd.h>
#include <fcntl.h>
#include <linux/i2c-dev.h>
#include <sys/ioctl.h>
#include <time.h>

uint64_t monotonic_nanoseconds() {
  struct timespec ts;
  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (uint64_t)ts.tv_sec * 1000000000U + (uint64_t)ts.tv_nsec;
}

int select_register(int file, uint8_t reg) {
  if (write(file, &reg, 1) != 1) {
    printf("failed to send address to read\n");
    return 1;
  }
  return 0;
}

int main() {
  int file = open("/dev/i2c-1", O_RDWR);

  if (file < 0) {
    printf("failed to open bus\n");
    return 1;
  }

  if (ioctl(file, I2C_SLAVE, 0xf)) {
    printf("failed to select device\n");
    return 1;
  }

  uint8_t setup[] =
    {
     // continous 14-bit measurement at 1kHz
     0x1b, 0b11011000,

     0x5c, 0x00,
     0x5d, 0x00,

     // turn on DRDY pin
     0x1c, 0x0c,

     // start measuring
     0x1d, 0x40,
    };

  if (write(file, setup, sizeof(setup)) != sizeof(setup)) {
    printf("failed to write magnetometer settings\n");
    return 1;
  }

  uint8_t drdy = 0;
  while (!drdy) {
    if (select_register(file, 0x18) || read(file, &drdy, 1) != 1) {
      printf("failed to read drdy");
      return 1;
    }
  }

  uint64_t prev_drdy = monotonic_nanoseconds();
  int16_t buf[3];
  while (1) {
    if (select_register(file, 0x10) || read(file, buf, 6) != 6) {
      printf("failed to read xyz\n");
      return 1;
    }
    printf("%i %i %i\n", buf[0], buf[1], buf[2]);

    prev_drdy += 1000000;
    struct timespec waittime = {0};
    waittime.tv_nsec = prev_drdy - monotonic_nanoseconds();
    struct timespec trash;
    nanosleep(&waittime, &trash);
  }
}

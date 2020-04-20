#ifndef ARDUINO

#include "frequency_extraction.h"
#include <iostream>

int main() {
  Comb<double, 50> comb;
  Resonator<double, 50> resonator(1);

  for (int i = 0; i < 1000; i++) {
    std::cout << sqrt(resonator.process(comb.process(sin(i / 25.0 * M_PI)))) << std::endl;
  }
}

#endif

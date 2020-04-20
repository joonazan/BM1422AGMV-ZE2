#ifndef ARDUINO

#include "frequency_extraction.h"
#include <iostream>

int main() {
  Comb<double, 100> comb;
  Resonator<double, 100> resonator(1.0/50);

  for (int i = 0; i < 1000; i++) {
    std::cout << sqrt(resonator.process(comb.process(6.4*sin(i / 25.0 * M_PI)))) << std::endl;
  }
}

#endif

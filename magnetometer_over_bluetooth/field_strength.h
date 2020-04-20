#pragma once
#include "frequency_extraction.h"
#include "reading.h"

constexpr uint32_t N = 200;

struct FieldStrengthsSquared {
  double data[4] = {0};
};

class FieldStrengthExtractor {
  Comb<int16_t, N> combs[3];
  Resonator<double, N> resonators[3][4] =
    {
     {
      {45.0 / 1000.0},
      {65.0 / 1000.0},
      {80.0 / 1000.0},
      {95.0 / 1000.0},
     },
     {
      {45.0 / 1000.0},
      {65.0 / 1000.0},
      {80.0 / 1000.0},
      {95.0 / 1000.0},
     },
     {
      {45.0 / 1000.0},
      {65.0 / 1000.0},
      {80.0 / 1000.0},
      {95.0 / 1000.0},
     }
    };

  FieldStrengthsSquared process(Reading reading) {
    FieldStrengthsSquared out;

    for (size_t axis = 0; axis < 3; axis++) {
      double combed = (double) combs[axis].process(reading.data[axis]);
      for (size_t freq = 0; freq < 4; freq++) {
        out.data[freq] += resonators[axis][freq].process(combed);
      }
    }

    return out;
  }
};

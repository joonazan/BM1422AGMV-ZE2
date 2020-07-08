#pragma once
#include "frequency_extraction.h"
#include "reading.h"

constexpr uint32_t DELAY = 200;
constexpr float SAMPLE_RATE = 1000.0;

#define RESONATOR_TEMPLATE \
  { \
   {30.0 / SAMPLE_RATE}, \
   {50.0 / SAMPLE_RATE}, \
   {70.0 / SAMPLE_RATE}, \
   {90.0 / SAMPLE_RATE}, \
  }

struct FieldStrengthsSquared {
  float data[4] = {0};
};

class FieldStrengthExtractor {
  Comb<int16_t, DELAY> combs[3];
  Resonator<float, DELAY> resonators[3][4] =
    {
     RESONATOR_TEMPLATE,
     RESONATOR_TEMPLATE,
     RESONATOR_TEMPLATE,
    };

public:
  FieldStrengthsSquared process(Reading reading) {
    FieldStrengthsSquared out;

    for (size_t axis = 0; axis < 3; axis++) {
      float combed = (float) combs[axis].process(reading.data[axis]);
      for (size_t freq = 0; freq < 4; freq++) {
        out.data[freq] += resonators[axis][freq].process(combed);
      }
    }

    return out;
  }
};

#include <stdint.h>
#include "sines.h"

class SineCounter {
  volatile uint16_t index{0};
  uint8_t *table;
  uint16_t len;
public:
  SineCounter(uint8_t *t, uint16_t l) : table(t), len(l) {}
  uint8_t next() {
    if (++index == len) {
      index = 0;
    }
    return table[index];
  }
};

#define SINE_COUNTER(name, data) \
  SineCounter name(data, sizeof(data))

SINE_COUNTER(s70, sine70Hz);
SINE_COUNTER(s45, sine45Hz);

void setup() {
  pinMode(3, OUTPUT);
  pinMode(11, OUTPUT);
  // Timer 2
  // Mode
  TCCR2A = _BV(COM2A1) | _BV(COM2B1) | _BV(WGM21) | _BV(WGM20);
  // Speed
  TCCR2B = _BV(CS20);
  // Duty cycles
  OCR2A = 128;
  OCR2B = 128;

  TIMSK2 = 1;
}

ISR(TIMER2_OVF_vect) {
  OCR2A = s45.next();
  OCR2B = s70.next();
}

void loop() {
}

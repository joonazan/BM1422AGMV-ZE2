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

SINE_COUNTER(s1, sine45Hz);
SINE_COUNTER(s2, sine65Hz);
SINE_COUNTER(s3, sine75Hz);
SINE_COUNTER(s4, sine90Hz);

void setup() {
  // Timer 2
  pinMode(11, OUTPUT);
  pinMode(3, OUTPUT);
  // Fast PWM, 0 to OCR2(A/B) is high
  TCCR2A = _BV(COM2A1) | _BV(COM2B1) | _BV(WGM21) | _BV(WGM20);
  // No prescaling
  TCCR2B = _BV(CS20);
  // Enable interrupt on timer overflow
  TIMSK2 = 1;

  // Timer 0
  pinMode(5, OUTPUT);
  pinMode(6, OUTPUT);
  // Fast PWM, 0 to OCR0(A/B) is high
  TCCR0A = _BV(COM0A1) | _BV(COM0B1) | _BV(WGM01) | _BV(WGM00);
  // No prescaling
  TCCR0B = _BV(CS00);
}

ISR(TIMER2_OVF_vect) {
  // Timer 2 PWM
  OCR2A = s1.next();
  OCR2B = s2.next();

  // Timer 0 PWM
  OCR0A = s3.next();
  OCR0B = s4.next();
}

void loop() {
}

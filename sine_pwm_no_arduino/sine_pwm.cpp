#include <stdint.h>
#include <math.h>
#include <avr/io.h>
#include <avr/pgmspace.h>
#include <avr/interrupt.h>

template <int N>
struct Table {
  uint8_t values[N];
};

constexpr int table_len(int freq) {
  return 16000000 / (256 * freq);
}

#define TWO_PI 6.283185307179586476925286766559
constexpr uint8_t table_at(int i, int N) {
  return (sin((float)i / (float)N * TWO_PI) * 0.5 + 0.5) * 255;
}

// Compile time function that constructs a sine lookup table of length N

template <int N>
constexpr Table<N> sine_table() {
  Table<N> t{};
  for (int i = 0; i < N; i++) {
    t.values[i] = table_at(i, N);
  }
  return t;
}

class SineLoop {
  volatile uint16_t index{0};
  const uint8_t* table;
  const uint16_t len;

public:
  SineLoop(const uint8_t* t, uint16_t l) : table(t), len(l) {}

  inline uint8_t next() {
    if (++index == len) {
      index = 0;
    }
    return pgm_read_byte(table + index);
  }
};

#define SINE_LOOP(name, freq) \
  static const PROGMEM Table<table_len(freq)> table##freq = sine_table<table_len(freq)>(); \
  static SineLoop name(table##freq.values, table_len(freq));

SINE_LOOP(s1, 45)
SINE_LOOP(s2, 65)
SINE_LOOP(s3, 80)
SINE_LOOP(s4, 95)

int main() {
  // Set pin 11 to output
  DDRB |= _BV(DDB3);
  // Set pins 3, 5 and 6 to output
  DDRD |= _BV(DDD3) | _BV(DDD5) | _BV(DDD6);

  // Timer 2
  // Fast PWM, 0 to OCR2(A/B) is high
  TCCR2A = _BV(COM2A1) | _BV(COM2B1) | _BV(WGM21) | _BV(WGM20);
  // No prescaling
  TCCR2B = _BV(CS20);
  // Enable interrupt on timer overflow
  TIMSK2 = 1;

  // Timer 0
  // Fast PWM, 0 to OCR0(A/B) is high
  TCCR0A = _BV(COM0A1) | _BV(COM0B1) | _BV(WGM01) | _BV(WGM00);
  // No prescaling
  TCCR0B = _BV(CS00);

  sei();
  while (true) {}
}

ISR(TIMER2_OVF_vect) {
  // Timer 2 PWM
  OCR2A = s1.next();
  OCR2B = s2.next();

  // Timer 0 PWM
  OCR0A = s3.next();
  OCR0B = s4.next();
}

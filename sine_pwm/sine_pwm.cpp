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

#define SINE_SETUP(name, reg, freq) \
  static const PROGMEM Table<table_len(freq)> table_##name = sine_table<table_len(freq)>(); \
  static constexpr const uint8_t *table_start_##name = table_##name.values; \
  static constexpr const uint8_t *table_end_##name = table_##name.values + table_len(freq); \
  static const uint8_t *table_ptr_##name = table_start_##name; \
  register uint8_t next_value_##name asm(reg);

#define SINE_STEP(name) \
  next_value_##name = pgm_read_byte(table_ptr_##name); \
  table_ptr_##name++; \
  if (table_ptr_##name == table_end_##name) { \
    table_ptr_##name = table_start_##name; \
  }

SINE_SETUP(a, "r2", 30)
SINE_SETUP(b, "r3", 50)
SINE_SETUP(c, "r4", 70)
SINE_SETUP(d, "r5", 90)

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

ISR(TIMER2_OVF_vect, ISR_NAKED) {
  // Timer 2 PWM
  OCR2A = next_value_a;
  OCR2B = next_value_b;

  // Timer 0 PWM
  OCR0A = next_value_c;
  OCR0B = next_value_d;

  SINE_STEP(a);
  SINE_STEP(b);
  SINE_STEP(c);
  SINE_STEP(d);

  asm("reti");
}

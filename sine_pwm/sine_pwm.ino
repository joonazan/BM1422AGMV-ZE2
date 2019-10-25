#include <stdint.h>

// I need to define enable_if, as there is no STL for AVR

template<bool B, class T = void>
struct enable_if {};

template<class T>
struct enable_if<true, T> { typedef T type; };


template <int N>
struct Table {
  uint8_t values[N];
};

constexpr int table_len(int freq) {
  return 16000000 / (256 * freq);
}

constexpr uint8_t table_at(int i, int N) {
  return (sin((float)i / (float)N * 2 * PI) * 0.5 + 0.5) * 255;
}

// Compile time code that constructs a sine lookup table of length N
// The maximum template nesting is 900 in the Arduino IDE, which is
// why I did not simply use the simplest possible recursion scheme
// This would be a lot easier in C++14, so building without Arduino
// should be considered.

template <int N, uint8_t ...Vals>
constexpr
typename enable_if<N==sizeof...(Vals),Table<N>>::type
sine_table() {
  return {{Vals...}};
}

template <int N, uint8_t ...Vals>
constexpr
typename enable_if<N-1==sizeof...(Vals), Table<N>>::type
sine_table() {
  return sine_table<N, Vals..., table_at(sizeof...(Vals), N)>();
}

template <int N, uint8_t ...Vals>
constexpr
typename enable_if<(N-sizeof...(Vals) > 1), Table<N>>::type
sine_table() {
  return sine_table<N, Vals..., table_at(sizeof...(Vals), N), table_at(sizeof...(Vals)+1, N)>();
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
  //OCR0A = s3.next();
  //OCR0B = s4.next();
}

void loop() {
}

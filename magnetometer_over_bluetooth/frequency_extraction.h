#pragma once

#include <stdint.h>
#include <math.h>

template <typename T, uint32_t size>
class RingBuffer {
  T buffer[size] = {0};
  uint32_t current = 0;
public:
  void insert(T val) {
    current = (current + 1) % size;
    buffer[current] = val;
  }
  T get_last() {
    return buffer[(current + 1) % size];
  }
};

// You only need one Comb if all you Resonators are the same size.
// The Comb only remembers the recent past, so it never needs to be restarted.
template <typename T, uint32_t size>
class Comb {
  RingBuffer<T, size> rb;
public:
  T process(T input) {
    auto res = input - rb.get_last();
    rb.insert(input);
    return res;
  }
};

// The Resonator finds the square of the magnitude in a given DFT bucket.
// It must be fed with values from a Comb of the same size.
//
// The bucket 1 is for waves that span `size` samples. #2 has twice the frequency
// #3 three times etc.
// bucket_frequency = bucket_no * samplerate / size
//
// The Resonator may accumulate some error as values propagate infinitely.
// TODO find out the amount of error. Test if subtracting an ULP like proposed in the original paper helps.
template <typename T, uint32_t size>
class Resonator {
  const T factor_re;
  const T factor_im;
  T previous_output_re = 0;
  T previous_output_im = 0;
public:
  Resonator(T k) :
    factor_re(cos(2*M_PI*k/size)),
    factor_im(sin(2*M_PI*k/size)) {}

  T process(T input) {
    auto re = previous_output_re * factor_re - previous_output_im * factor_im + input;
    auto im = previous_output_re * factor_im + previous_output_im * factor_re;

    previous_output_re = re;
    previous_output_im = im;

    return re*re + im*im;
  }
};

#pragma once

// Portable implementation of Sliding DFT
// See "The sliding DFT" in IEEE Signal Processing Magazine, April 2003

#include <stdint.h>
#include <math.h>

template <typename T, uint32_t size>
class FixedSizeQueue {
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
  FixedSizeQueue<T, size> rb;
public:
  T process(T input) {
    auto res = input - rb.get_last();
    rb.insert(input);
    return res;
  }
};

// The Resonator finds a certain frequency span's amplitude squared.
// The span's width is sample rate / `size`.
// The constructor argument is the desired frequency divided by the sample rate.
//
// The Resonator must be fed with values from a Comb of the same size.
// The first `size` outputs are garbage and should be ignored.
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
  Resonator(T freq) :
    factor_re(cos(2*M_PI*freq)),
    factor_im(sin(2*M_PI*freq)) {}

  T process(T input) {
    const auto re = previous_output_re * factor_re - previous_output_im * factor_im + input;
    const auto im = previous_output_re * factor_im + previous_output_im * factor_re;

    previous_output_re = re;
    previous_output_im = im;

    const auto re_scaled = re / size * 2;
    const auto im_scaled = im / size * 2;
    return re_scaled*re_scaled + im_scaled*im_scaled;
  }
};

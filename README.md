# Magnetic position tracking

Code for finding the position of a magnetometer or a coil relative to electromagnets in a fixed arrangement.

The process consists of two phases: determining each magnet's field strength at the measuring device and computing the position of the measuring device based on that.

## Finding each magnet's field strength at the position of interest

The main problem is isolating a single magnet's field from the earth's magnetic field, noise and the other magnets. This is not possible with just one point of data. One has to have multiple points of measurement or a sequence of measurements.

This project drives each magnet with a different sine wave. One only has to isolate the correct frequency from the magnetic field measurements.

### On magnetometers

Magnetometers are very small and thus easy to attach and unobtrusive. They could even be part of some cyberpunk nail art. But it is hard to find a suitable one for this purpose. I suspect that is because they are mostly meant for mobile phones.

The BM1422AGMV-ZE2 is the best one I could find. It can be set up to measure with 14-bit accuracy at one kilohertz.

The accuracy directly affects the accuracy of the position. Note that the difference between a magnet's positive and negative peak gets smaller and smaller with distance, so only a small part of the scale may be in use at one time.

The sample rate affects latency and accuracy while moving. Think about the Discrete Fourier Transform. With eight samples one would get one frequency bucket per magnet, the bare minimum. With more samples noise is filtered out.

We know the phase of the sine waves, which may help do more with less samples. I haven't investigated this.

One more pain point of magnetometers reading the data. The BM1422AGMV-ZE only supports two different I²C addresses, so each pair of them takes up one bus. They only support a 400kHz clock, too, so the bus is saturated.

### Isolating the frequencies

The signal processing happens on the chip that reads the magnetometers, as it is computationally inexpensive and reduces the amount of data by an order of magnitude. The code for the microprocessor is in `magnetometer_over_bluetooth`. (TODO name it better)

A magnetometer measures the magnetic field on three perpendicular axes. The readings are not centered around zero magnetic field and changing the hardware in the slightest offsets them. Because of that, we cannot compute the unfiltered field strength with the Pythagorean Theorem. Each axis is filtered separately and then combined. This happens in `field_strength.h`.

The filtering is done using an algorithm that keeps a single DFT bucket up to date. Updating the bucket based on a new sample is O(1) regardless of DFT size. The buckets can be centered around any value but their window width depends on DFT size as usual. "The sliding DFT" (IEEE Signal Processing Magazine, April 2003) is a very detailed and readable explanation of the algorithm.

### Using a coil instead?

The receivers could be coils. Coils are larger than magnetometers and can have a bit more personality. However, with the correct circuit, they are not limited by a sample rate. That means that they can receive very high frequencies, which means low latency and low noise.

## Generating sines

`sine_pwm_no_arduino` contains the code that I've used on my Arduino Uno to generate sine waves via PWM. `flash.sh` is what I used to flash. The commands should be pretty straightforward to adapt to any platform.

I don't use the Arduino IDE because its compiler flags prevent me from generating a sine table in a sane way. `sine_pwm` is the same but uses ugly templates for sine table generation.

Both versions use Timer 0 and 2 and fire 250 000 interrupts per second to get very high frequency PWM. That may interfere with using the Arduino for other purposes.

The output needs to be smoothed with a low-pass filter and amplified.

A sine generator circuit could be used instead but it probably wouldn't even be cheaper than a cheap SoC. When using coils as receivers, a sine circuit is superior because this method can only go up to about 1 kHz with good quality.

## Finding the position

A point is uniquely determined by its distance from n points, where n is the number of dimensions plus one. For example in two dimensions two circles can intersect at two points, but three different circles can only intersect at one point.

`amplitude_to_position` is used to test and compare different algorithms for finding the point from which the magnet's strengths were measured.

There would be a simple exact solution if the fields spread out in a sphere (as is the case with GPS). However, only magnetic monopoles would have a spherical field. The field of a coil has a circular cross-section but the equation for its radius is fourth-degree.

TODO insert image of field shape

The same testbed can also be used to automatically compare different placements of magnets and thus find an optimal placement for a given level of noise and placement constraints.

TODO Some kind of Gradient descent probably works but I haven't tried it yet
TODO What about partitioning the cube root of field strength space?

### What doesn't work

The following two methods work perfectly if the measurements are perfectly accurate. But for high accuracy in practice a method is required that reacts gracefully to error.

#### Space partitioning

Implemented in `octtree.rs`. Finds the smallest cube that contains points with the correct field strength for each magnet.

Split cubes into smaller cubes. Keep the cubes that are still good.

Bad because:

- Splitting the optimal cube creates two cubes that are rejected.
- Performance is relatively bad and unpredictable. May allocate a lot. Performance may be better with a depth-first traversal rather than breadth-first.

#### Slicing

Implemented in `slicer.rs`. Slices the problem into a bunch of very cheaply solved circle intersection problems, takes the slice with least error.

The biggest possible slope of the error vs. z is `4 / cbrt(2)` because that is the worst that could happen when moving on the z-axis and the best solution either has the same x and y or has less error.

Bad because:

- Can only support configurations where all magnets point the same way.
- The slices' solutions are not optimal. It turns out that growing each circle has a different (maybe even nonlinear) cost. Meanwhile, the solutions minimize squared error in radius.

There is still some hope for this if there is a way to find the best solution to a slice.

## Running the whole stack

The Arduino MKR Wifi 1010 can be flashed with the Arduino IDE. FastBLE and I2C_DMAC are required because the official ones can't be used asynchronously. FastBLE includes a part that needs to be flashed onto the ESP side of the board. See its README.

Two BM1422AGMV-ZE magnetometers with address select wired differently can be attached to pins 11 (SDA here) and 12 (SCL here). TODO update once the other two pins pairs are used as well. (Should make it detect what pins have a device on them)

TODO write about computer side after I've worked with it again

import numpy as np
import sys
from scipy import fftpack

points = []
with open(sys.argv[1], 'rb') as f:
    for line in f:
        points.append(list(map(int, line.split())))

'''
The dft bins are centered around n * sampling_frequency / number_of_samples
Here we have conveniently selected a number of samples equal to the sampling
frequency.
'''

axes = np.array([
    np.array(points)[:, 0],
    np.array(points)[:, 1],
    np.array(points)[:, 2]
])

def field_strengths_for_one_axis(x):
    strengths = np.abs(fftpack.fft(x))
    return [strengths[45], strengths[70]]

for lo in range(0, len(axes[0]) - 1000, 10):
    field_strengths = np.sum(np.square(np.array(list(map(field_strengths_for_one_axis, axes[:, lo:lo+1000])))), axis=0)
    distances = field_strengths**(-1/6)
    print(*distances)

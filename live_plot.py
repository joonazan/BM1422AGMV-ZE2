import numpy as np
import sys
from scipy import fftpack

def field_strengths(axes):
    def field_strengths_for_one_axis(x):
        strengths = np.abs(fftpack.fft(x)) / 1000
        return [strengths[45], strengths[70]]

    field_strengths = np.sum(np.square(np.array(list(map(field_strengths_for_one_axis, axes)))), axis=0)
    return field_strengths

while True:
    points = []
    for _ in range(1000):
        points.append(list(map(int, input().split())))

    axes = np.array([
        np.array(points)[:, 0],
        np.array(points)[:, 1],
        np.array(points)[:, 2]
    ])

    K = 1
    distances = K * field_strengths(axes)**(-1/6)
    print(*distances)

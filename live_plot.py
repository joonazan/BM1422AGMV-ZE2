import numpy as np
from scipy import fftpack
import matplotlib.pyplot as plt

def field_strengths(axes):
    def field_strengths_for_one_axis(x):
        strengths = np.abs(fftpack.fft(x)) / 1000
        return [strengths[45], strengths[65], strengths[80]]

    field_strengths = np.sum(np.square(np.array(list(map(field_strengths_for_one_axis, axes)))), axis=0)
    return field_strengths

fig, ax = plt.subplots()
plt.xlim(-1, 1)
plt.ylim(-1, 1)
ax.set_aspect(1)

circles = [
    plt.Circle((0,0), fill=False, radius=0.1),
    plt.Circle((1.8,0), fill=False, radius=0.1),
    plt.Circle((0.9, 0.21), fill=False, radius=0.1)
]

for circle in circles:
    ax.add_artist(circle)

while True:
    points = []
    for _ in range(1000):
        points.append(list(map(int, input().split())))

    axes = np.array([
        np.array(points)[:, 0],
        np.array(points)[:, 1],
        np.array(points)[:, 2]
    ])

    K = 2
    distances = K * field_strengths(axes)**(-1/6)

    for c, d in zip(circles, distances):
        c.set_radius(d)

    plt.pause(0.001)

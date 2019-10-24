import numpy as np
from scipy import fftpack
import matplotlib.pyplot as plt

def field_strengths(axes):
    def field_strengths_for_one_axis(x):
        strengths = np.abs(fftpack.fft(x)) / 1000
        return [strengths[45], strengths[70]]

    field_strengths = np.sum(np.square(np.array(list(map(field_strengths_for_one_axis, axes)))), axis=0)
    return field_strengths

fig, ax = plt.subplots()
plt.xlim(-1, 1)
plt.ylim(-1, 1)
ax.set_aspect(1)

circle = plt.Circle((0,0), fill=False, radius=0.1)
circle2 = plt.Circle((0.175,0), fill=False, radius=0.1)

ax.add_artist(circle)
ax.add_artist(circle2)

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

    circle.set_radius(distances[0])
    circle2.set_radius(distances[1])
    plt.pause(0.001)

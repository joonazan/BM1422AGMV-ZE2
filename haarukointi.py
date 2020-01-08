import numpy as np
from scipy import fftpack
import matplotlib.pyplot as plt

def field_strengths(axes):
    def field_strengths_for_one_axis(x):
        strengths = np.abs(fftpack.fft(x)) / 1000
        return [strengths[45], strengths[65], strengths[80]]

    field_strengths = np.sum(np.square(np.array(list(map(field_strengths_for_one_axis, axes)))), axis=0)
    return field_strengths

positions = ((0, 0), (5, 0), (5, 5))

def field_strength_at_point((x, y)):
    dist = np.sqrt(x**2 + y**2)
    return 50 * dist**(-6)

def field_strength_range_in_rect((left, right, top, bottom)):
    corners = list(map(field_strength_at_point, ((left, top), (right, top), (left, bottom), (right, bottom))))
    min_H = min(corners)
    max_H = max(corners)

    if left < 0 < right and top < 0 < bottom:
        max_H = float("inf")
    elif left < 0 < right:
        max_H = max(
            max_H,
            field_strength_at_point((0, top)),
            field_strength_at_point((0, bottom))
        )
    elif top < 0 < bottom:
        max_H = max(
            max_H,
            field_strength_at_point((left, 0)),
            field_strength_at_point((right, 0))
        )

    return min_H, max_H

def subdivide((left, right, top, bottom)):
    mid_x = (left + right) / 2
    mid_y = (top + bottom) / 2

    return (
        (left, mid_x, top, mid_x),
        (mid_x, right, top, mid_x),
        (mid_x, right, mid_x, bottom),
        (left, mid_x, mid_x, bottom)
    )

def offset((left, right, top, bottom), (x, y)):
    return (left + x, right + x, top + y, bottom + y)

points = []
for _ in range(1000):
    points.append(list(map(int, input().split())))

axes = np.array([
    np.array(points)[:, 0],
    np.array(points)[:, 1],
    np.array(points)[:, 2]
])

strengths = field_strengths(axes)

def ok(rect):
    for p, s in zip(positions, strengths):
        lo, hi = field_strength_range_in_rect(offset(rect, p))
        if not lo < s < hi:
            return False
    return True

rects = [(-50, 50, -50, 50)]

while True:
    rects = [sr for r in rects for sr in subdivide(r) if ok(sr)]
    print(rects)

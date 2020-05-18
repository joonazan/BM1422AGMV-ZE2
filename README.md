# Magnetic position tracking

Code related to tracking the position of a magnetometer or a coil relative to electromagnets in a fixed arrangement.

`amplitude_to_position` is used to test and compare different algorithms for finding the point from which the magnet's strengths were measured.

There would be a simple exact solution if the fields spread out in a sphere (as is the case with GPS). However, only magnetic monopoles would have a spherical field. The field of a coil has a circular cross-section but the equation for radius of the circle is fourth-degree.

The same testbed can also be used to automatically compare different placements of magnets and thus find an optimal placement for a given level of noise and placement constraints.

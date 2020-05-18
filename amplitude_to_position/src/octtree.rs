#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;
#[cfg(test)]
#[macro_use]
extern crate float_cmp;

use crate::interface::*;

use std::ops::RangeInclusive;
type AABB = RangeInclusive<Vec3>;

fn field_strength_range(bb: AABB) -> RangeInclusive<f64> {
    // The critical points of the field strength can only be on the extrema
    // or on projections of the origin (magnet center).
    //
    // Critical point locations:
    // - the origin
    // - projection of the origin onto a face
    // - projection of the origin onto an edge
    // - the vertices

    // The maximum is at the closest critical point.
    // This is fairly easy to see for the case where that point is on a face
    // and the magnet's north is aligned with one of the axes.
    // I have proved most interesting cases but presenting a rigorous proof
    // here would is too tedious.

    fn closest_to_zero(start: f64, end: f64) -> f64 {
        if start <= 0.0 && 0.0 <= end {
            0.0
        } else if start > 0.0 {
            start
        } else {
            end
        }
    }

    let max = field_strength(Vec3::new(
        closest_to_zero(bb.start().x, bb.end().x),
        closest_to_zero(bb.start().y, bb.end().y),
        closest_to_zero(bb.start().z, bb.end().z),
    ));

    // I have proven that the minimum must be at a vertex.
    // The furthest vertex seems intuitively correct.

    fn max_abs(a: f64, b: f64) -> f64 {
        if a.abs() > b.abs() {
            a
        } else {
            b
        }
    }

    let min = field_strength(Vec3::new(
        max_abs(bb.start().x, bb.end().x),
        max_abs(bb.start().y, bb.end().y),
        max_abs(bb.start().z, bb.end().z),
    ));

    min..=max
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute_force_field_strength_range(bb: AABB) -> Range<f64> {
        let xs = vec![bb.start().x, bb.end().x, 0.0].into_iter();
        let ys = vec![bb.start().y, bb.end().y, 0.0].into_iter();
        let zs = vec![bb.start().z, bb.end().z, 0.0].into_iter();

        let criticals = xs
            .flat_map(|x| {
                ys.clone().flat_map({
                    let zs = &zs;
                    move |y| zs.clone().map(move |z| Vec3::new(x, y, z))
                })
            })
            .filter(|c| bb.contains(c))
            .map(field_strength);

        criticals
            .clone()
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()..criticals.max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()
    }

    #[quickcheck]
    fn field_strength_range_same_as_checking_all_critical_points(start: Vec3, size: f64) -> bool {
        let size = size.abs();
        let bb = start..=start + Vec3::new(size, size, size);

        let actual = field_strength_range(bb.clone());
        let correct = brute_force_field_strength_range(bb);

        approx_eq!(f64, actual.start, correct.start, ulps = 10)
            && approx_eq!(f64, actual.end, correct.end, ulps = 10)
    }
}

fn subdivide(bb: &AABB) -> impl Iterator<Item = AABB> {
    let off = 0.5 * (bb.end() - bb.start());
    let base = *bb.start()..=bb.start() + off;
    vec![0.0, off.x].into_iter().flat_map(move |off_x| {
        let b = base.clone();
        vec![0.0, off.y].into_iter().flat_map(move |off_y| {
            let b2 = b.clone();
            vec![0.0, off.z]
                .into_iter()
                .map(move |off_z| offset(&b2, &Vec3::new(off_x, off_y, off_z)))
        })
    })
}

fn offset(bb: &AABB, v: &Vec3) -> AABB {
    bb.start() + v..=bb.end() + v
}

pub struct Octtree {
    search_area: RangeInclusive<Vec3>,
    offsets: Vec<Vec3>,
}

impl AmplitudesToPosition for Octtree {
    fn new(magnet_positions: [Vec3; 4], max_distance: f64) -> Self {
        Self {
            search_area: Vec3::new(-max_distance, -max_distance, -max_distance)
                ..=Vec3::new(max_distance, max_distance, max_distance),
            offsets: magnet_positions.iter().map(|x| -x).collect(),
        }
    }
    fn locate(&self, amplitudes_squared: [f64; 4]) -> Vec3 {
        let mut rects = vec![self.search_area.clone()];

        for _ in 0..35 {
            let new: Vec<AABB> = rects
                .iter()
                .flat_map(subdivide)
                .filter(|rect| {
                    for (o, s) in self.offsets.iter().zip(&amplitudes_squared) {
                        if !field_strength_range(offset(rect, o)).contains(s) {
                            return false;
                        }
                    }
                    true
                })
                .collect();

            if new.is_empty() {
                break;
            }
            rects = new;
        }

        (rects[0].start() + rects[0].end()) / 2.0
    }
}

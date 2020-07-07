use crate::interface::*;

/// Solves (z² + r²)^4 - r² - 4z² = 0, which is the equation relating height and
/// radius when the field strength is one.
///
/// In polar coordinates, the equation relating field strength and position is
/// 2H² * r^6 = 5 - 3 cos(2x)
/// By solving for r, we can see that r follows
/// the inverse cube root of field strength.
///
/// Note that the r of the polar representation is not the radius we are looking for.
/// However, we can see that the inverse cube root of the field strength
/// uniformly scales everything, so we only need to scale this function's input and
/// output to get values for all field strengths.
pub fn radius(z: f64) -> f64 {
    let z = z.abs();

    // I am using Halley's Method instead of Newton's because Newton's
    // converges only asymptotically on a zero with zero derivative. The
    // zero we are looking for is at x=0 and has a zero derivative when we
    // are at the very top of the capsule.
    //
    // An alternative would be to use Newton's and have some special case
    // for the edge case. Halleys seems pretty good, though as it converges
    // in two iterations for many cases.

    // 1 is a good starting point for Newton's method.
    // 1 is the largest possible radius and a big part is very close to it.
    // Approaching from 1 is safe, unlike the other side; the derivative is
    // zero at a point between the intersection with the x-axis and zero.
    let mut r = 1.0;
    let z2 = z * z;
    for _ in 0..20 {
        let r2 = r * r;
        let a = r2 + z2;
        let a2 = a * a;
        let a3 = a2 * a;
        let a4 = a2 * a2;
        let f = a4 - r2 - 4.0 * z2;
        let df = 8.0 * r * a3 - 2.0 * r;
        let d2f = 8.0 * a3 + 48.0 * r2 * a2 - 2.0;
        r -= (2.0 * f * df) / (2.0 * df * df - f * d2f);

        // When the function only barely touches zero, r oscillates around it
        // instead of converging.
        r = r.max(0.0);
    }

    r
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO this fails with very small x and very large z
    // should make the epsilon somehow related to them.
    #[quickcheck]
    fn radius_computed_correctly(x: f64, z: f64) -> bool {
        let x = x.abs();
        let h = field_strength([x, 0.0, z].into());
        let inv_scale = h.powf(1.0 / 6.0);
        x == 0.0 && z == 0.0
            || approx_eq!(
                f64,
                radius(z * inv_scale) / inv_scale,
                x,
                epsilon = 0.00000001
            )
    }

    #[quickcheck]
    fn intersection_point_correct(positions: nalgebra::Vector4<Vec2>, point: Vec2) -> bool {
        let mut dists = [0.0; 4];
        let mut ps = [Vec2::new(0.0, 0.0); 4];
        for i in 0..4 {
            dists[i] = (positions[i] - point).norm();
            ps[i] = positions[i];
        }
        let computed = PositionOptimizer::new(&ps).best_pos(&dists);
        approx_eq!(f64, (point - computed).norm_squared(), 0.0)
    }
}

type Vec2 = nalgebra::Vector2<f64>;

/// Takes four points and does some precomputation. Then, it can quickly
/// find a point based on distance to those four points. It minimizes squared error.
///
/// Based on multilateration from Localization in Wireless Sensor Networks.
struct PositionOptimizer {
    qt: nalgebra::Matrix2x3<f64>,
    offset: Vec2,
    ymul: f64,
}

impl PositionOptimizer {
    fn new(centers: &[Vec2; 4]) -> Self {
        let n = nalgebra::Matrix2x4::from_columns(centers);

        let a = (n.fixed_columns::<nalgebra::U3>(1)
            - nalgebra::Matrix2x3::from_columns(&[n.column(0); 3]))
        .transpose();

        let (q, r) = a.qr().unpack();
        let a = 2.0 * r;

        let mut qt = q.transpose();
        for x in qt.row_mut(0).iter_mut() {
            *x /= a[0];
        }
        for x in qt.row_mut(1).iter_mut() {
            *x /= a[3];
        }

        let b_fixed_part = Vec3::from_iterator(
            (1..=3).map(|i| n.column(i).norm_squared() - n.column(0).norm_squared()),
        );

        Self {
            qt,
            offset: qt * b_fixed_part,
            ymul: a[2] / a[0],
        }
    }

    fn best_pos(&self, radii: &[f64; 4]) -> Vec2 {
        let b = self.qt
            * Vec3::from_iterator((1..=3).map(|i| radii[0] * radii[0] - radii[i] * radii[i]))
            + self.offset;

        let y = b[1];
        let x = b[0] - self.ymul * y;
        Vec2::new(x, y)
    }
}

pub struct NaiveSlicer {
    magnet_positions: [Vec3; 4],
    planar_solver: PositionOptimizer,
}

impl AmplitudesToPosition for NaiveSlicer {
    fn new(magnet_positions: [Vec3; 4], _: f64) -> Self {
        let mut without_z = [Vec2::new(0.0, 0.0); 4];
        for i in 0..4 {
            without_z[i] = magnet_positions[i].xy();
        }
        Self {
            magnet_positions,
            planar_solver: PositionOptimizer::new(&without_z),
        }
    }

    fn locate(&self, amplitudes_squared: [f64; 4]) -> Vec3 {
        // The surface with a certain field strength is capsule shaped.
        // It is always the same shape but scaled by the inverse cube root of field strength.
        // See radius for details.
        // Its top and bottom are at cube root of two.

        let mut scale = [0.0; 4];
        let mut inv_scale = [0.0; 4];
        for i in 0..4 {
            inv_scale[i] = amplitudes_squared[i].powf(1.0 / 6.0);
            scale[i] = 1.0 / inv_scale[i];
        }

        // The highest bottom and lowest top delimit the search space
        let mut minz = std::f64::MIN;
        let mut maxz = std::f64::MAX;
        for (scale, center) in scale.iter().zip(&self.magnet_positions) {
            let half_height = 2f64.cbrt() * scale;
            let new_bottom = center.z - half_height;
            let new_top = center.z + half_height;
            if new_bottom > minz {
                minz = new_bottom;
            }
            if new_top < maxz {
                maxz = new_top;
            }
        }

        let iters = 10000;
        let mut radii = [0.0; 4];
        (0..iters)
            .map(|i| {
                let z = minz + i as f64 * (maxz - minz) / (iters - 1) as f64;
                for i in 0..4 {
                    radii[i] = scale[i] * radius(inv_scale[i] * (z - self.magnet_positions[i].z));
                }
                let xy = self.planar_solver.best_pos(&radii);
                Vec3::new(xy.x, xy.y, z)
            })
            .min_by_key(|pos| {
                let mut error = 0.0;
                for i in 0..4 {
                    let v = field_strength(pos - self.magnet_positions[i]);
                    error += (amplitudes_squared[i] - v).abs();
                }
                float_ord::FloatOrd(error)
            })
            .unwrap()
    }
}

#[cfg(test)]
mod full_test {
    use super::*;

    #[quickcheck]
    fn works_for_exact_measurements(p: Vec3) -> bool {
        let magnet_positions = [
            [0.0, 0.0, 0.0].into(),
            [0.5, 0.0, 0.0].into(),
            [0.5, 0.5, 0.0].into(),
            [0.0, 0.5, 0.3].into(),
        ];
        let mut asq = [0.0; 4];
        for i in 0..4 {
            asq[i] = field_strength(p - magnet_positions[i]);
        }
        let p2 = NaiveSlicer::new(magnet_positions, 100.0).locate(asq);
        println!("{} {}", p, p2);
        approx_eq!(f64, (p - p2).norm_squared(), 0.0, epsilon = 0.001)
    }
}

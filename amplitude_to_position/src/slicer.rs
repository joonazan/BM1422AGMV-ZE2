use crate::interface::*;

pub fn radius(z: f64, amplitude_squared: f64) -> f64 {
    // solves H² * (z² + r²)^4 - r² - 4z² = 0 for r

    // In polar coordinates, the equation for field strength is
    // 2H² * r^6 = 5 - 3 cos(2x)
    // By solving for r, we can see that r follows
    // the inverse cube root of field strength.
    //
    // Note that the r of the polar representation is not the radius we are looking for.
    // However, we can see that the inverse cube root of the field strength
    // uniformly scales everything.
    let inv_scale = amplitude_squared.powf(1.0/6.0);
    let scale = 1.0 / inv_scale;
    let z = (z * inv_scale).abs();

    // TODO this info is relevant for intelligently choosing what heights to check
    // (z² + r²)^4 - r² - 4z² = 0 is capsule shaped.
    // height of the top:
    // (z² + 0)^4 - 0 - 4z² = 0
    // z^6 = 4z²
    // z^3 = 2

    // Newton's method
    // 1 is a good starting point.
    // 1 is the largest possible radius and a big part is very close to it.
    // Approaching from 1 is safe, unlike the other side; the derivative is
    // zero at a point between the intersection with the x-axis and zero.
    let mut r = 1.0;
    let z2 = z * z;
    for _ in 0..1000 {
        let r2 = r * r;
        let a = r2 + z2;
        let a2 = a * a;
        let a3 = a2 * a;
        let a4 = a2 * a2;
        let f = a4 - r2 - 4.0 * z2;
        let df = 8.0 * r * a3 - 2.0*r;
        let df2 = 0;
        r -= f / df;
    }

    r * scale
}

#[cfg(test)]
mod tests {
    use super::*;

    #[quickcheck]
    fn radius_computed_correctly(x: f64, z: f64) -> bool {
        let x = x.abs();
        let h = field_strength([x, 0.0, z].into());
        x == 0.0 && z == 0.0 || approx_eq!(f64, radius(z, h), x, epsilon = 0.0000000001)
    }
}

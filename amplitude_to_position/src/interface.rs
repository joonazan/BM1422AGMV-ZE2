use nalgebra::Vector3;

pub type Vec3 = Vector3<f64>;

pub fn field_strength(p: Vec3) -> f64 {
    let r2 = p.norm_squared();
    if r2 == 0.0 {
        // We can interpret zero as approaching zero from the positive side
        std::f64::INFINITY
    } else {
        let cos_times_norm = p.z;
        (3.0 * cos_times_norm * cos_times_norm / r2 + 1.0) / (r2 * r2 * r2)
    }
}

pub trait AmplitudesToPosition {
    fn new(magnet_positions: [Vec3; 4], max_distance: f64) -> Self
    where
        Self: Sized;
    fn locate(&self, amplitudes_squared: [f64; 4]) -> Vec3;
}

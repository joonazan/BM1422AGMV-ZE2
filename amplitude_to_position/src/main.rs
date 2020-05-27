#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;
#[cfg(test)]
#[macro_use]
extern crate float_cmp;

mod interface;
mod octtree;
mod slicer;
use crate::interface::*;
use octtree::Octtree;
use rand_distr::{Distribution, Normal, UnitBall};
use slicer::NaiveSlicer;

fn main() {
    let max_dist = 10.0;
    let magnet_positions = [
        [0.0, 0.0, 0.0].into(),
        [0.5, 0.0, 0.0].into(),
        [0.5, 0.5, 0.0].into(),
        [0.0, 0.5, 0.3].into(),
    ];

    let noise = Normal::new(0.0, 0.00001).unwrap();

    let ot = Octtree::new(magnet_positions, max_dist);
    let ns = NaiveSlicer::new(magnet_positions, max_dist);

    let mut rng = rand::thread_rng();
    let real_pos: Vec3 = UnitBall.sample(&mut rng).into();
    let real_pos = max_dist * real_pos;

    let mut measurements = [0.0; 4];
    for i in 0..4 {
        let actual_field_strength_squared = field_strength(real_pos - magnet_positions[i]);
        let with_noise = actual_field_strength_squared.sqrt() + noise.sample(&mut rng);
        measurements[i] = with_noise * with_noise;
    }

    let judge = |m: &dyn AmplitudesToPosition| (real_pos - m.locate(measurements)).norm_squared();

    println!("squared error");
    println!("octtree {}", judge(&ot));
    println!("slicer  {}", judge(&ns));
}

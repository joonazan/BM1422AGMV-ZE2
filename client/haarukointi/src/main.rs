use nalgebra::{Vector2, Vector3};
use piston_window::WindowSettings;
use plotters::prelude::*;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
mod config;

#[cfg(test)]
extern crate quickcheck;
#[cfg(test)]
#[macro_use]
extern crate quickcheck_macros;
#[cfg(test)]
#[macro_use]
extern crate float_cmp;

struct NewestFieldStrengths(Arc<Mutex<Vec<f64>>>);

impl NewestFieldStrengths {
    pub fn new() -> Self {
        let me = NewestFieldStrengths(Arc::new(Mutex::new(vec![0.0; 4])));
        me.start();
        me
    }
    pub fn get(&self) -> Vec<f64> {
        self.0.lock().unwrap().clone()
    }
    fn start(&self) {
        let s = self.0.clone();
        std::thread::spawn(move || loop {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).expect("read error");
            let x = line
                .split_whitespace()
                .map(|x| x.parse::<f64>().expect("parse error"))
                .collect::<Vec<f64>>();

            *s.lock().unwrap() = x;
        });
    }
}

type Vec3 = Vector3<f64>;

use std::ops::Range;
type AABB = std::ops::RangeInclusive<Vec3>;

fn field_strength(p: Vec3) -> f64 {
    let r2 = p.norm_squared();
    if r2 == 0.0 {
        // We can interpret zero as approaching zero from the positive side
        std::f64::INFINITY
    } else {
        let cos_times_norm = p.z;
        (3.0 * cos_times_norm * cos_times_norm / r2 + 1.0) / (r2 * r2 * r2)
    }
}

fn field_strength_range(bb: AABB) -> Range<f64> {
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

    min..max
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

fn main() {
    let mut window = WindowSettings::new("Possible Locations", [1024, 768])
        .build()
        .unwrap();

    let config = config::load_config();

    let magnet_positions = config
        .magnets
        .iter()
        .map(|x| x.position.into())
        .collect::<Vec<Vec3>>();
    let offsets: Vec<Vec3> = magnet_positions.iter().map(|x| -x).collect();
    let frequencies: Vec<usize> = config.magnets.iter().map(|x| x.frequency).collect();

    let search_area = Vec3::new(
        -config.max_distance,
        -config.max_distance,
        -config.max_distance,
    )
        ..=Vec3::new(
            config.max_distance,
            config.max_distance,
            config.max_distance,
        );

    const STRENGTH_PLOT_LEN: usize = 100;
    let mut previous_strengths: Vec<VecDeque<f64>> = vec![vec![0.0; 100].into(); frequencies.len()];

    let field_strengths_squared = NewestFieldStrengths::new();

    while draw_piston_window(&mut window, |b| {
        let root = b.into_drawing_area();
        root.fill(&WHITE).unwrap();
        let views = root.split_evenly((2, 2));
        let left_view = &views[0];
        let right_view = &views[1];

        let draw_rect = |start, end, style, view: &DrawingArea<_, _>| {
            let convert = |pos: Vector2<f64>| {
                let mul = 700.0 / (config.max_distance * 2.0);
                let pos = pos * mul;
                (pos.x as i32, pos.y as i32)
            };

            let r = plotters::prelude::Rectangle::new([convert(start), convert(end)], style);
            view.draw(&r).unwrap();
        };

        let draw_aabb = |aabb: &AABB, color| {
            let aabb = offset(aabb, &Vec3::from_element(config.max_distance));

            let style = ShapeStyle {
                color,
                filled: false,
                stroke_width: 2,
            };

            draw_rect(aabb.start().xy(), aabb.end().xy(), style.clone(), left_view);
            draw_rect(aabb.start().xz(), aabb.end().xz(), style, right_view);
        };

        draw_aabb(&search_area, BLACK.to_rgba());
        magnet_positions.iter().for_each(|s| {
            let r = Vec3::from_element(config.max_distance) / 100.0;
            draw_aabb(&((s - r)..=(s + r)), BLACK.to_rgba());
        });

        let mut strengths = field_strengths_squared.get();
        for s in &mut strengths {
            *s /= config.magnet_strength;
        }

        let mut strength_chart = ChartBuilder::on(&views[2])
            .margin(10)
            .caption("Magnet strengths", ("sans-serif", 30).into_font())
            .x_label_area_size(40)
            .y_label_area_size(50)
            .build_ranged(0usize..STRENGTH_PLOT_LEN, 0.0..25.0)?;

        strength_chart
            .configure_mesh()
            .x_labels(15)
            .y_labels(5)
            .draw()?;

        for i in 0..frequencies.len() {
            previous_strengths[i].pop_front();
            previous_strengths[i].push_back(strengths[i].log2());

            strength_chart
                .draw_series(LineSeries::new(
                    previous_strengths[i].iter().cloned().enumerate(),
                    &Palette99::pick(i),
                ))?
                .label(format!("{} Hz", frequencies[i]))
                .legend(move |(x, y)| {
                    PathElement::new(vec![(x, y), (x + 20, y)], &Palette99::pick(i))
                });
        }

        strength_chart.configure_series_labels().draw()?;

        let mut rects = vec![search_area.clone()];

        for _ in 0..100 {
            let new: Vec<AABB> = rects
                .iter()
                .flat_map(subdivide)
                .filter(|rect| {
                    for (o, s) in offsets.iter().zip(&strengths) {
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

        for r in rects {
            draw_aabb(&r, BLUE.to_rgba());
        }

        Ok(())
    })
    .is_some()
    {}
}

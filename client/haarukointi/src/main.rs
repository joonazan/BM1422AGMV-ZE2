use nalgebra::{Vector2, Vector3};
use piston_window::WindowSettings;
use plotters::prelude::*;
use rustfft::num_complex::Complex;
use rustfft::num_traits::{Pow, Zero};
use rustfft::FFTplanner;
use std::collections::VecDeque;
mod config;

fn get_field_strengths_squared(frequencies: &[usize]) -> Vec<f64> {
    let mut axes = [[Complex::zero(); 1000]; 3];

    for i in 0..1000 {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).expect("read error");
        let nums = line
            .split_whitespace()
            .map(|x| x.parse::<f64>().expect("parse error"))
            .collect::<Vec<f64>>();

        for j in 0..3 {
            axes[j][i] = Complex::new(nums[j], 0.0);
        }
    }

    let fft = FFTplanner::new(false).plan_fft(1000);
    let mut out = [Complex::zero(); 1000];

    let mut sum = vec![0.0; frequencies.len()];
    for a in &mut axes {
        fft.process(a, &mut out);
        for (i, x) in frequencies
            .iter()
            .map(|&i| (out[i] / 1000.0).norm_sqr())
            .enumerate()
        {
            sum[i] += x;
        }
    }

    sum
}

type Vec3 = Vector3<f64>;

use std::ops::Range;
type AABB = Range<Vec3>;

fn field_strength(p: Vec3) -> f64 {
    let r = p.norm();
    if r == 0.0 {
        // We can interpret zero as approaching zero from the positive side
        std::f64::INFINITY
    } else {
        let sine = (p.z / r).sin();
        r.pow(-6) * (3.0 * sine * sine + 1.0)
    }
}

fn field_strength_range(bb: AABB) -> Range<f64> {
    // I have proven that the critical points of the field strength can only
    // be on the extrema or on projections of the origin (magnet center).
    //
    // We need to consider all combinations of those:
    // - the origin
    // - projection of the origin onto a face
    // - projection of the origin onto an edge
    // - the vertices

    let bb2 = bb.clone();
    let vertices = vec![bb.start.x, bb.end.x].into_iter().flat_map(move |x| {
        let bb3 = bb2.clone();
        vec![bb2.start.y, bb2.end.y].into_iter().flat_map(move |y| {
            vec![bb3.start.z, bb3.end.z]
                .into_iter()
                .map(move |z| Vec3::new(x, y, z))
        })
    });

    fn max_field_strength(it: impl Iterator<Item = Vec3>) -> f64 {
        it.map(field_strength)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap()
    }

    let zero_in_x = (bb.start.x..bb.end.x).contains(&0.0);
    let zero_in_y = (bb.start.y..bb.end.y).contains(&0.0);
    let zero_in_z = (bb.start.z..bb.end.z).contains(&0.0);

    let max = match (zero_in_x, zero_in_y, zero_in_z) {
        (true, true, true) => std::f64::INFINITY,

        (true, true, false) => max_field_strength(
            [bb.start.z, bb.end.z]
                .iter()
                .map(|&z| Vec3::new(0.0, 0.0, z)),
        ),
        (true, false, true) => max_field_strength(
            [bb.start.y, bb.end.y]
                .iter()
                .map(|&y| Vec3::new(0.0, y, 0.0)),
        ),
        (false, true, true) => max_field_strength(
            [bb.start.x, bb.end.x]
                .iter()
                .map(|&x| Vec3::new(x, 0.0, 0.0)),
        ),

        (true, false, false) => {
            max_field_strength(vec![bb.start.y, bb.end.y].into_iter().flat_map(|y| {
                vec![bb.start.z, bb.end.z]
                    .into_iter()
                    .map(move |z| Vec3::new(0.0, y, z))
            }))
        }
        (false, true, false) => {
            max_field_strength(vec![bb.start.x, bb.end.x].into_iter().flat_map(|x| {
                vec![bb.start.z, bb.end.z]
                    .into_iter()
                    .map(move |z| Vec3::new(x, 0.0, z))
            }))
        }
        (false, false, true) => {
            max_field_strength(vec![bb.start.x, bb.end.x].into_iter().flat_map(|x| {
                vec![bb.start.y, bb.end.y]
                    .into_iter()
                    .map(move |y| Vec3::new(x, y, 0.0))
            }))
        }
        (false, false, false) => max_field_strength(vertices.clone()),
    };

    vertices
        .map(field_strength)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()..max
}

fn subdivide(bb: &AABB) -> impl Iterator<Item = AABB> {
    let off = 0.5 * (bb.end - bb.start);
    let base = bb.start..(bb.start + off);
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
    bb.start + v..bb.end + v
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
        ..Vec3::new(
            config.max_distance,
            config.max_distance,
            config.max_distance,
        );

    const STRENGTH_PLOT_LEN: usize = 100;
    let mut previous_strengths: Vec<VecDeque<f64>> = vec![vec![0.0; 100].into(); frequencies.len()];

    while let Some(_) = draw_piston_window(&mut window, |b| {
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

            draw_rect(aabb.start.xy(), aabb.end.xy(), style.clone(), left_view);
            draw_rect(aabb.start.xz(), aabb.end.xz(), style, right_view);
        };

        draw_aabb(&search_area, BLACK.to_rgba());
        magnet_positions.iter().for_each(|s| {
            let r = Vec3::from_element(config.max_distance) / 100.0;
            draw_aabb(&((s - r)..(s + r)), BLACK.to_rgba());
        });

        let mut strengths = get_field_strengths_squared(&frequencies);
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

        for i in 0..100 {
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
    }) {}
}

use amplitude_to_position::{interface::{Vec3, AmplitudesToPosition}, slicer::NaiveSlicer};
use nalgebra::{Vector2};
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

type AABB = std::ops::RangeInclusive<Vec3>;

fn offset(bb: &AABB, v: &Vec3) -> AABB {
    bb.start() + v..=bb.end() + v
}

fn main() {
    let mut window = WindowSettings::new("Possible Locations", [1024, 768])
        .build()
        .unwrap();

    let config = config::load_config();

    let mut magnet_positions = [Vec3::new(0.0, 0.0, 0.0); 4];
    for (i, x) in config.magnets.iter().map(|x| x.position.into()).enumerate() {
        magnet_positions[i] = x;
    }

    let frequencies: Vec<usize> = config.magnets.iter().map(|x| x.frequency).collect();

    const STRENGTH_PLOT_LEN: usize = 100;
    let mut previous_strengths: Vec<VecDeque<f64>> = vec![vec![0.0; 100].into(); frequencies.len()];

    let field_strengths_squared = NewestFieldStrengths::new();

    // Change to the second line to use the space partitioning method
    let slicer = NaiveSlicer::new(magnet_positions, config.max_distance);
    //let slicer = amplitude_to_position::octtree::Octtree::new(magnet_positions, config.max_distance);

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

        magnet_positions.iter().for_each(|s| {
            let r = Vec3::from_element(config.max_distance) / 100.0;
            draw_aabb(&((s - r)..=(s + r)), BLACK.to_rgba());
        });

        let mut strengths = field_strengths_squared.get();
        for s in &mut strengths {
            *s /= config.magnet_strength;
        }

        let mut st = [0.0; 4];
        for i in 0..4 {
            st[i] = strengths[i];
        }
        let pos = slicer.locate(st);
        draw_aabb(&(pos ..= pos + Vec3::new(0.04, 0.04, 0.04)), GREEN.to_rgba());

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

        Ok(())
    })
    .is_some()
    {}
}

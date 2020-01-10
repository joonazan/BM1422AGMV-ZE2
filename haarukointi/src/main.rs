use piston_window::WindowSettings;
use plotters::prelude::*;
use rustfft::num_complex::Complex;
use rustfft::num_traits::{Pow, Zero};
use rustfft::FFTplanner;
use std::collections::VecDeque;
mod config;

fn get_field_strengths_squared(frequencies: &[usize]) -> [f64; 3] {
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

    let mut sum = [0.0; 3];
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

#[derive(Debug, Clone)]
struct Rectangle {
    start: Complex<f64>,
    end: Complex<f64>,
}

fn field_strength(p: &Complex<f64>) -> f64 {
    p.norm_sqr().sqrt().pow(-6)
}

impl Rectangle {
    fn contains_field_strength(&self, waldo: f64) -> bool {
        let corners = [
            self.start,
            self.end,
            Complex::new(self.start.re, self.end.im),
            Complex::new(self.end.re, self.start.im),
        ]
        .iter()
        .map(field_strength)
        .collect::<Vec<f64>>();
        if waldo
            < *corners
                .iter()
                .min_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap()
        {
            return false;
        }

        let zero_in_x = self.start.re < 0.0 && 0.0 < self.end.re;
        let zero_in_y = self.start.im < 0.0 && 0.0 < self.end.im;

        zero_in_x && zero_in_y
            || zero_in_x
                && (waldo < field_strength(&Complex::new(0.0, self.start.im))
                    || waldo < field_strength(&Complex::new(0.0, self.end.im)))
            || zero_in_y
                && (waldo < field_strength(&Complex::new(self.start.re, 0.0))
                    || waldo < field_strength(&Complex::new(self.end.re, 0.0)))
            || waldo
                < *corners
                    .iter()
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap()
    }

    fn subdivide(&self) -> Vec<Rectangle> {
        let mid_x = (self.start.re + self.end.re) / 2.0;
        let mid_y = (self.start.im + self.end.im) / 2.0;

        vec![
            Rectangle {
                start: self.start,
                end: Complex::new(mid_x, mid_y),
            },
            Rectangle {
                start: Complex::new(self.start.re, mid_y),
                end: Complex::new(mid_x, self.end.im),
            },
            Rectangle {
                start: Complex::new(mid_x, mid_y),
                end: self.end,
            },
            Rectangle {
                start: Complex::new(mid_x, self.start.im),
                end: Complex::new(self.end.re, mid_y),
            },
        ]
    }

    fn offset(&self, v: &Complex<f64>) -> Rectangle {
        Rectangle {
            start: self.start + v,
            end: self.end + v,
        }
    }
}

fn main() {
    let mut window = WindowSettings::new("Possible Locations", [1024, 768])
        .build()
        .unwrap();

    let config = config::load_config();

    let magnet_positions = config.magnets.iter().map(|x| Complex::new(x.position.0, x.position.1)).collect::<Vec<Complex<f64>>>();
    let offsets: Vec<Complex<f64>> = magnet_positions.iter().map(|x| -x).collect();
    let frequencies: Vec<usize> = config.magnets.iter().map(|x| x.frequency).collect();

    let search_area = Rectangle {
        start: Complex::new(-config.max_distance, -config.max_distance),
        end: Complex::new(config.max_distance, config.max_distance),
    };

    let mut previous_positions: VecDeque<Vec<Rectangle>> = vec![vec![]; 20].into();

    while let Some(_) = draw_piston_window(&mut window, |b| {
        let root = b.into_drawing_area();
        root.fill(&WHITE).unwrap();
        let root = root.shrink(((1024 - 700) / 2, 0), (700, 700));

        let draw_rect = |r: &Rectangle, color| {
            let style = ShapeStyle {
                color,
                filled: false,
                stroke_width: 2,
            };

            let convert = |x: Complex<f64>| {
                let mul = 700.0 / (config.max_distance * 2.0);
                let x = x * mul;
                let offset = config.max_distance * mul;
                ((x.re + offset) as i32, (x.im + offset) as i32)
            };

            let r = plotters::prelude::Rectangle::new([convert(r.start), convert(r.end)], style);
            root.draw(&r).unwrap();
        };

        draw_rect(&search_area, BLACK.to_rgba());
        magnet_positions.iter().for_each(|s| {
            let r = Complex::new(0.2, 0.2);
            draw_rect(
                &Rectangle {
                    start: s - r,
                    end: s + r,
                },
                BLACK.to_rgba(),
            );
        });

        let mut strengths = get_field_strengths_squared(&frequencies);
        for s in &mut strengths {
            *s /= config.magnet_strength;
        }
        let mut rects = vec![search_area.clone()];

        for i in 0..100 {
            let new: Vec<Rectangle> = rects
                .iter()
                .flat_map(|x| x.subdivide().into_iter())
                .filter(|rect| {
                    for (o, s) in offsets.iter().zip(&strengths) {
                        if !rect.offset(o).contains_field_strength(*s) {
                            return false;
                        }
                    }
                    true
                })
                .collect();

            if new.is_empty() {
                println!("{:?}", i);
                break;
            }
            rects = new;
        }

        previous_positions.pop_front();
        previous_positions.push_back(rects);

        for rs in &previous_positions {
            for r in rs {
                draw_rect(&r, BLUE.to_rgba());
            }
        }

        Ok(())
    }) {}
}

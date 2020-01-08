use arrayvec::ArrayVec;
use rustfft::num_complex::Complex;
use rustfft::num_traits::{Pow, Zero};
use rustfft::FFTplanner;

fn field_strengths_for_one_axis(xs: &mut [Complex<f64>]) -> ArrayVec<[f64; 3]> {
    let fft = FFTplanner::new(false).plan_fft(1000);
    let mut out = [Complex::zero(); 1000];
    fft.process(xs, &mut out);

    [45, 65, 80]
        .iter()
        .map(|&i| out[i].norm_sqr() / 1000.0)
        .collect::<ArrayVec<[_; 3]>>()
}

fn get_field_strengths_squared() -> [f64; 3] {
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

    let mut sum = [0.0; 3];
    for a in &mut axes {
        for (i, x) in field_strengths_for_one_axis(a).iter().enumerate() {
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
    100000000000.0 * p.norm_sqr().sqrt().pow(-6)
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
            Rectangle{start: self.start, end: Complex::new(mid_x, mid_y)},
            Rectangle{start: Complex::new(self.start.re, mid_y), end: Complex::new(mid_x, self.end.im)},
            Rectangle{start: Complex::new(mid_x, mid_y), end: self.end},
            Rectangle{start: Complex::new(mid_x, self.start.im), end: Complex::new(self.end.re, mid_y)},
        ]
    }

    fn offset(&self, v: &Complex<f64>) -> Rectangle {
        Rectangle{start: self.start + v, end: self.end + v}
    }
}

fn main() {
    let strengths = get_field_strengths_squared();
    println!("{:?}", strengths);
    let offsets: [Complex<f64>; 3] = [Complex::new(0.0, 0.0), Complex::new(-5.0, 0.0), Complex::new(-5.0, -5.0)];

    let mut rects = vec![Rectangle{start: Complex::new(-50.0, -50.0), end: Complex::new(50.0, 50.0)}];

    for _ in 0..10 {
        rects = rects.iter().flat_map(|x| x.subdivide().into_iter()).filter(|rect|{
            for (o, s) in offsets.iter().zip(&strengths) {
                if !rect.offset(o).contains_field_strength(*s){
                    return false;
                }
            }
            true
        }).collect();
        println!("{:?}", rects);
    }
}

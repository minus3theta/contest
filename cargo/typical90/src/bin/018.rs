#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

#[fastout]
fn main() {
    input! {
        t: f64,
        l: f64,
        x: f64,
        y: f64,
        q: usize,
        es: [f64; q],
    }
    for &e in &es {
        let angle = 2.0 * std::f64::consts::PI * e / t;
        let cam = (0.0, -0.5 * l * angle.sin(), 0.5 * l * (1.0 - angle.cos()));
        let h_dist = ((cam.0 - x).powi(2) + (cam.1 - y).powi(2)).sqrt();
        println!("{:.15}", cam.2.atan2(h_dist).to_degrees());
    }
}

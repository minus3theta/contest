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

use num::Complex;

#[derive(Clone)]
struct Coord {
    origin: Complex<i64>,
    x: Complex<i64>,
    y: Complex<i64>,
}

impl Coord {
    fn new() -> Self {
        Self {
            origin: Complex::new(0, 0),
            x: Complex::new(1, 0),
            y: Complex::new(0, 1),
        }
    }
    fn rotate(&self, rot: Complex<i64>) -> Self {
        Self {
            origin: self.origin * rot,
            x: self.x * rot,
            y: self.y * rot,
        }
    }
    fn flip_x(&self, p: i64) -> Self {
        Self {
            origin: -self.origin.conj() + Complex::new(p * 2, 0),
            x: -self.x.conj(),
            y: -self.y.conj(),
        }
    }
    fn flip_y(&self, p: i64) -> Self {
        Self {
            origin: self.origin.conj() + Complex::new(0, 2 * p),
            x: self.x.conj(),
            y: self.y.conj(),
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        point: [(i64, i64); n],
        m: usize,
    }
    let mut origins = vec![Coord::new()];
    for _ in 0..m {
        input! {
            op: i32,
        }
        let tail = origins.last().unwrap().clone();
        let org = match op {
            1 => tail.rotate(Complex::new(0, -1)),
            2 => tail.rotate(Complex::new(0, 1)),
            3 => {
                input! {
                    p: i64,
                }
                tail.flip_x(p)
            }
            4 => {
                input! {
                    p: i64,
                }
                tail.flip_y(p)
            }
            _ => unreachable!(),
        };
        origins.push(org);
    }
    input! {
        q: usize,
        query: [(usize, Usize1); q],
    }
    for &(a, b) in &query {
        let (x, y) = point[b];
        let conv = &origins[a];
        let ans = conv.origin + conv.x * x + conv.y * y;
        println!("{} {}", ans.re, ans.im);
    }
}

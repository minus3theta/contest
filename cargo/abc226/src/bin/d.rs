#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
use num_integer::gcd;
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

type Coord = (i64, i64);

fn magic(p: Coord, q: Coord) -> (Coord, Coord) {
    let (x, y) = (p.0 - q.0, p.1 - q.1);
    let gcd = gcd(x, y);
    let (x, y) = (x / gcd, y / gcd);
    ((x, y), (-x, -y))
}

#[fastout]
fn main() {
    input! {
        n: usize,
        city: [(i64, i64); n],
    }
    let mut set = HashSet::new();
    for (&p, &q) in city.iter().tuple_combinations() {
        let (m1, m2) = magic(p, q);
        set.insert(m1);
        set.insert(m2);
    }

    println!("{}", set.len());
}

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

#[derive(Debug, Eq, PartialEq, Clone, Ord, PartialOrd, Default)]
struct Vertex {
    price: i64,
    children: Vec<usize>,
}

impl Vertex {
    fn new(price: i64) -> Self {
        Self {
            price,
            ..Default::default()
        }
    }
}

const INF: i64 = 1 << 60;

fn chmax<T: Ord + Copy>(y: T, x: &mut T) {
    *x = cmp::max(*x, y);
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        es: [(Usize1, Usize1); m],
    }
    let mut vs = a.into_iter().map(Vertex::new).collect_vec();
    for &(a, b) in &es {
        vs[a].children.push(b);
    }
    let mut max_price = vec![-INF; n];
    let mut max_gain = vec![-INF; n];
    let mut ans = -INF;
    for (i, v) in vs.iter().enumerate().rev() {
        for &j in &v.children {
            chmax(max_price[j], &mut max_price[i]);
            chmax(max_gain[j], &mut max_gain[i]);
        }
        chmax(max_price[i] - v.price, &mut max_gain[i]);
        chmax(v.price, &mut max_price[i]);
        chmax(max_gain[i], &mut ans);
    }
    println!("{}", ans);
}

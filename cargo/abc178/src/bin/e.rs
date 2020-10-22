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
        n: usize,
        ps: [(i64, i64); n],
    }
    let cheb: Vec<_> = ps.into_iter().map(|(x, y)| (x + y, x - y)).collect();
    use itertools::MinMaxResult::MinMax;
    let da = match cheb.iter().minmax_by_key(|&(a, _)| a) {
        MinMax((min_a, _), (max_a, _)) => max_a - min_a,
        _ => unreachable!(),
    };
    let db = match cheb.iter().minmax_by_key(|&(_, b)| b) {
        MinMax((_, min_b), (_, max_b)) => max_b - min_b,
        _ => unreachable!(),
    };

    println!("{}", cmp::max(da, db));
}

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
        y: [(i32,i32,i32); 2],
        x: i32,
    }
    fn dist((a, b, c): (i32, i32, i32), x: i32) -> i32 {
        let cycle = x / (a + c);
        let rest = (x % (a + c)).min(a);
        b * (cycle * a + rest)
    }

    println!(
        "{}",
        match dist(y[0], x).cmp(&dist(y[1], x)) {
            cmp::Ordering::Less => "Aoki",
            cmp::Ordering::Equal => "Draw",
            cmp::Ordering::Greater => "Takahashi",
        }
    );
}

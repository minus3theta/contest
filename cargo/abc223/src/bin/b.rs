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
        s: Chars,
    }
    let n = s.len();
    let mut s = s.into_iter().collect::<VecDeque<_>>();
    let mut min = s.clone();
    let mut max = s.clone();
    for _ in 0..n {
        let c = s.pop_front().unwrap();
        s.push_back(c);
        if s < min {
            min = s.clone();
        }
        if s > max {
            max = s.clone();
        }
    }

    println!(
        "{}\n{}",
        min.into_iter().collect::<String>(),
        max.into_iter().collect::<String>()
    );
}

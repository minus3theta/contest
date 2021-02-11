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

#[fastout]
fn main() {
    input! {
        _n: usize,
        m: usize,
    }
    let odd = m / 2;
    let even = m - odd;
    let offset = even * 2 + 1;
    for e in 1..=even {
        println!("{} {}", e, offset - e);
    }
    for o in 1..=odd {
        println!("{} {}", even * 2 + o, even * 2 + odd * 2 + 2 - o);
    }
}

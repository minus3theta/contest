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
        n: isize,
        rc: [(Isize1, Isize1); n-1],
    }
    let mut rs = HashSet::new();
    let mut cs = HashSet::new();
    let mut add = HashSet::new();
    let mut sub = HashSet::new();
    for &(r, c) in &rc {
        rs.insert(r);
        cs.insert(c);
        add.insert(r + c);
        sub.insert(r - c);
    }
    let ans = (0..n).cartesian_product(0..n).find(|&(r, c)| {
        !rs.contains(&r) && !cs.contains(&c) && !add.contains(&(r + c)) && !sub.contains(&(r - c))
    });
    if let Some((r, c)) = ans {
        println!("{} {}", r + 1, c + 1);
    } else {
        println!("-1");
    }
}

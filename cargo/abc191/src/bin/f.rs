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
        a: [i64; n],
    }
    let ub = *a.iter().min().unwrap();
    let mut t = HashMap::new();
    let mut update = |d: i64, x: i64| {
        t.entry(d)
            .and_modify(|v| *v = num::integer::gcd(*v, x))
            .or_insert(x);
    };
    for &x in &a {
        for div in 1i64.. {
            if div.pow(2) > x {
                break;
            }
            if x % div != 0 {
                continue;
            }
            update(div, x);
            update(x / div, x);
        }
    }
    let ans = t.iter().filter(|(&k, &v)| k == v && k <= ub).count();
    println!("{}", ans);
}

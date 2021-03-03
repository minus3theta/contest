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

fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        return (a, 1, 0);
    }
    let (q, r) = a.div_mod_floor(&b);
    let (g, x, y) = ext_gcd(b, r);
    // bx + ry = g
    // bx + (a - bq)y = g
    // ay + b(x - qy) = g
    assert_eq!(a * y + b * (x - q * y), g);
    (g, y, x - q * y)
}

fn solve(m1: i64, m2: i64, b1: i64, b2: i64) -> Option<i64> {
    // _ * m + a = _ * n + b
    // _ * m - _ * n = b - a
    let (d, p, _) = ext_gcd(m1, m2);
    let (quo, r) = (b2 - b1).div_mod_floor(&d);
    if r != 0 {
        return None;
    }
    // x * m + y * n = g
    // qx * m + qy * n = gq = b - a
    // assert_eq!(quo * p * m1 + b1, -quo * q * m2 + b2);
    let m = m1 * m2 / d;
    let ret = (quo * p % (m2 / d) * m1 + b1).rem_euclid(m);
    // assert!(ret >= 0);
    // assert_eq!(ret % m1, b1);
    // assert_eq!(ret % n, b);
    return Some(ret);
}

#[fastout]
fn main() {
    input! {
        t: usize,
        cases: [(i64, i64, i64, i64); t],
    }
    for &(x, y, p, q) in &cases {
        let m = 2 * (x + y);
        let n = p + q;
        let mut ans = None::<i64>;
        for a in 0..y {
            for b in 0..q {
                if let Some(time) = solve(m, n, x + a, p + b) {
                    if let Some(o) = ans.as_mut() {
                        *o = cmp::min(*o, time);
                    } else {
                        ans = Some(time);
                    }
                }
            }
        }
        if let Some(ans) = ans {
            println!("{}", ans);
        } else {
            println!("infinity");
        }
    }
}

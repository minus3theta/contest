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

fn pow(x: i64, y: i64, m: i64) -> i64 {
    if y == 0 {
        1
    } else if y % 2 == 1 {
        x * pow(x, y - 1, m) % m
    } else {
        let p = pow(x, y / 2, m);
        p * p % m
    }
}

fn cycle(b: i64, c: i64) -> i64 {
    let b = b % 4;
    let p = pow(b, c, 4);
    if p == 0 {
        4
    } else {
        p
    }
}

#[fastout]
fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    }
    let a = a % 10;
    let p = cycle(b, c);
    let ans = pow(a, p, 10);
    println!("{}", ans);
}

#[test]
fn test_pow4() {
    for i in 0i64..10 {
        assert_eq!(i, i.pow(5) % 10);
    }
}

#[test]
fn test_p() {
    assert_eq!(cycle(2, 1), 2);
    assert_eq!(cycle(2, 2), 4);
    assert_eq!(cycle(2, 3), 4);
    assert_eq!(cycle(2, 4), 4);
    assert_eq!(cycle(3, 1), 3);
    assert_eq!(cycle(3, 2), 1);
    assert_eq!(cycle(3, 3), 3);
}

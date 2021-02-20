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

fn eval(x: &[i64], base: i64) -> Option<i64> {
    let mut ret = 0i64;
    for &d in x {
        if let Some(r) = ret.checked_mul(base) {
            ret = r;
        } else {
            return None;
        }
        ret += d;
    }
    Some(ret)
}

fn solve(x: &Vec<char>, m: i64) -> i64 {
    let x = x.into_iter().map(|&c| c as i64 - '0' as i64).collect_vec();
    if x.len() == 1 {
        let v = x[0];
        return if v <= m { 1 } else { 0 };
    }
    let d = *x.iter().max().unwrap();
    let mut l = d;
    let mut r = m + 20;
    while l + 1 != r {
        let base = (l + r) / 2;
        match eval(&x, base) {
            Some(v) if v <= m => {
                l = base;
            }
            _ => {
                r = base;
            }
        }
    }
    l - d
}

#[fastout]
fn main() {
    input! {
        x: Chars,
        m: i64,
    }
    println!("{}", solve(&x, m));
}

#[test]
fn test_eval() {
    assert_eq!(eval(&[2, 2], 3), Some(8));
    assert_eq!(eval(&[2, 2], 4), Some(10));
    assert_eq!(
        eval(&[2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 100000000),
        None
    )
}

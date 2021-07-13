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

fn to_digits(mut x: i64) -> Vec<i64> {
    let mut ret = vec![];
    while x > 0 {
        ret.push(x % 10);
        x /= 10;
    }
    ret.sort();
    ret
}

fn is_ok(n: i64, b: i64, v: &[i64]) -> i64 {
    let prod: i64 = v.iter().product();
    let m = prod + b;
    if m <= n && v == to_digits(m).as_slice() {
        dbg!(v);
        1
    } else {
        0
    }
}

fn search(n: i64, b: i64, mut v: Vec<i64>, max: i64, max_len: usize) -> i64 {
    let mut ret = 0;
    if v.len() < max_len {
        if max < 9 {
            ret += search(n, b, v.clone(), max + 1, max_len);
        }
        v.push(max);
        ret += is_ok(n, b, &v);
        ret += search(n, b, v, max, max_len);
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: i64,
        b: i64,
    }
    let mut max_len = 0;
    let mut x = n;
    while x > 0 {
        x /= 10;
        max_len += 1;
    }

    println!("{}", search(n, b, vec![], 0, max_len));
}

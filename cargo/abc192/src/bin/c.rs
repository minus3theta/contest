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

fn decompose(mut x: i64) -> Vec<i64> {
    let mut digits = vec![];
    while x != 0 {
        digits.push(x % 10);
        x /= 10;
    }
    digits
}

fn compose(digits: &Vec<i64>) -> i64 {
    let mut ret = 0;
    for &d in digits {
        ret *= 10;
        ret += d;
    }
    ret
}

fn g1(mut digits: Vec<i64>) -> i64 {
    digits.sort();
    digits.reverse();
    compose(&digits)
}

fn g2(mut digits: Vec<i64>) -> i64 {
    digits.sort();
    compose(&digits)
}

fn f(x: i64) -> i64 {
    let digits = decompose(x);
    g1(digits.clone()) - g2(digits)
}

#[fastout]
fn main() {
    input! {
        mut n: i64,
        k: i32,
    }
    for _ in 0..k {
        n = f(n);
    }
    println!("{}", n);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_g1() {
        assert_eq!(g1(decompose(314)), 431);
        assert_eq!(g1(decompose(3021)), 3210);
        assert_eq!(g1(decompose(10)), 10);
        assert_eq!(g1(decompose(1)), 1);
    }
    #[test]
    fn test_g2() {
        assert_eq!(g2(decompose(3021)), 123);
        assert_eq!(g2(decompose(1)), 1);
    }
}

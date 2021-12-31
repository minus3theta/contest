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
        x: Chars,
    }
    let digits = x.iter().map(|d| d.to_digit(10).unwrap()).collect_vec();
    let mut sum: u32 = digits.iter().sum();
    let mut carry = 0;
    let mut ans = vec![];
    for &digit in digits.iter().rev() {
        let s = sum + carry;
        ans.push(s % 10);
        sum -= digit;
        carry = s / 10;
    }
    if carry != 0 {
        ans.push(carry);
    }

    for &a in ans.iter().rev() {
        print!("{}", a);
    }
    println!();
}

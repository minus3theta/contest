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
use std::error::Error;

fn op(n: &str) -> Result<String, Box<dyn Error>> {
    let mut value = u64::from_str_radix(n, 8)?;
    let mut nine = vec![];
    while value != 0 {
        nine.push(if value % 9 == 8 {
            '5'
        } else {
            std::char::from_digit((value % 9) as u32, 9).unwrap()
        });
        value /= 9;
    }

    if nine.is_empty() {
        Ok("0".to_owned())
    } else {
        Ok(nine.into_iter().rev().collect())
    }
}

#[fastout]
fn main() {
    input! {
        mut n: String,
        k: i32,
    }
    for _ in 0..k {
        n = op(&n).unwrap();
    }

    println!("{}", n);
}

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
        s: String,
    }
    for mut chunk in &s.chars().chunks(2) {
        let c = chunk.next().unwrap();
        let d = chunk.next().unwrap();
        print!("{}{}", d, c);
    }

    println!();
}

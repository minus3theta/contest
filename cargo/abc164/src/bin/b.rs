#[allow(unused_imports)]
use std::cmp;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::fastout;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
  input! {
    mut a: i32,
    b: i32,
    mut c: i32,
    d: i32,
  }
  loop {
    c -= b;
    if c <= 0 {
      println!("Yes");
      break;
    }
    a -= d;
    if a <= 0 {
      println!("No");
      break;
    }
  }
}

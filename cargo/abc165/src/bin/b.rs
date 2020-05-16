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

#[fastout]
fn main() {
  input! {
    x: i64,
  }
  let mut m = 100;
  for i in 0.. {
    if m >= x {
      println!("{}", i);
      break;
    }
    m += m / 100;
  }
}

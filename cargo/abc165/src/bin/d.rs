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
    a: i64,
    b: i64,
    n: i64,
  }
  let f = |x| a * x / b - a * (x / b);

  println!("{}", f(cmp::min(b - 1, n)));
}

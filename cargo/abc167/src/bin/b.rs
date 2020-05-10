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
    c: i64,
    mut k: i64,
  }
  let mut sum = 0;
  let ca = cmp::min(a, k);
  sum += ca;
  k -= ca;
  let cb = cmp::min(b, k);
  k -= cb;
  let cc = cmp::min(c, k);
  sum -= cc;

  println!("{}", sum);
}

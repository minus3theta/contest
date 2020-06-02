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

fn double_median(v: &mut Vec<i64>) -> i64 {
  v.sort();
  let n = v.len();
  if n % 2 == 0 {
    v[n / 2 - 1] + v[n / 2]
  } else {
    v[n / 2]
  }
}

#[fastout]
fn main() {
  input! {
    n: usize,
    mut ab: [(i64, i64); n],
  }
  let lb = double_median(&mut ab.iter().map(|&(a, _)| a).collect());
  let ub = double_median(&mut ab.iter().map(|&(_, b)| b).collect());

  println!("{}", ub - lb + 1);
}

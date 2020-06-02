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

fn solve(a: &Vec<i64>) -> Option<i64> {
  let mut p = 1i64;
  for &x in a {
    p = p.checked_mul(x)?;
    if p > 1_000_000_000_000_000_000 {
      return None;
    }
  }
  Some(p)
}

#[fastout]
fn main() {
  input! {
    n: usize,
    mut a: [i64; n],
  }
  a.sort();

  println!("{}", solve(&a).unwrap_or(-1));
}

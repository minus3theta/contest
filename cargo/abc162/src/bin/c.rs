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
    k: i64,
  }
  let mut sum = 0;
  for a in 1 ..= k {
    for b in 1 ..= k {
      let g = a.gcd(&b);
      for c in 1 ..= k {
        sum += g.gcd(&c);
      }
    }
  }

  println!("{}", sum);
}

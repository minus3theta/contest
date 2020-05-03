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
    n: usize,
    k: usize,
    a: [[Usize1]; k],
  }
  let mut target = vec![true; n];
  for t in a {
    for i in t {
      target[i] = false;
    }
  }

  println!("{}", target.iter().filter(|&&t| t).count());
}

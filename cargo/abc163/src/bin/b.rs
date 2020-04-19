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
    n: i64,
    m: usize,
    a: [i64; m],
  }
  let d: i64 = a.iter().sum();

  println!("{}", if d <= n { n - d } else { -1 });
}

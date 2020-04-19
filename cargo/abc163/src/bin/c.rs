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
    n: usize,
    a: [Usize1; n-1],
  }
  let mut c = vec![0; n];
  for &x in &a {
    c[x] += 1;
  }
  for &x in &c {
    println!("{}", x);
  }
}

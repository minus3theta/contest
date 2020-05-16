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
    k: i32,
    a: i32,
    b: i32,
  }
  if (a..=b).any(|x| x % k == 0) {
    println!("OK");
  } else {
    println!("NG");
  }
}

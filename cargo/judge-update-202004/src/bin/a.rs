#[allow(unused_imports)]
use std::cmp;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::fastout;
use num::Integer;

#[fastout]
fn main() {
  input! {
    s: i32,
    l: i32,
    r: i32,
  }

  println!("{}", cmp::max(cmp::min(s, r), l));
}

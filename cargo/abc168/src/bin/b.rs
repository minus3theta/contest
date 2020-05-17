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
    k: usize,
    s: Chars,
  }
  if s.len() <= k {
    for &c in &s {
      print!("{}", c);
    }
    println!("");
  } else {
    for &c in s.iter().take(k) {
      print!("{}", c);
    }
    println!("...");
  }
}

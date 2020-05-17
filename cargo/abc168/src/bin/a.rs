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
    n: i64,
  }

  println!(
    "{}",
    match n % 10 {
      2 | 4 | 5 | 7 | 9 => "hon",
      3 => "bon",
      _ => "pon",
    }
  );
}

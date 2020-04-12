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
  }
  let mut sum = 0;
  for i in 1 ..= n {
    if i % 3 != 0 && i % 5 != 0 {
      sum += i;
    }
  }

  println!("{}", sum);
}

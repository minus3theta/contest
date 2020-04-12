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
    n: Chars,
  }
  if n.iter().all(|&c| c != '7') {
    println!("No");
  } else {
    println!("Yes");
  }
}

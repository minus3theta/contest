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
    a: [i64; n],
  }
  use std::collections::BTreeMap;
  let mut sum = BTreeMap::new();
  let mut ans = 0i64;
  for (i, &x) in a.iter().enumerate() {
    let diff = i as i64 - x;
    ans += sum.get(&diff).unwrap_or(&0);
    *sum.entry(x + i as i64).or_insert(0) += 1;
  }

  println!("{}", ans);
}

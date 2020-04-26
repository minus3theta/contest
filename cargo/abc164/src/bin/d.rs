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

const M: usize = 2019;

#[fastout]
fn main() {
  input! {
    s: Chars,
  }
  let mut sum = vec![0; s.len() + 1];
  let mut pow = 1;
  for (i, &c) in s.iter().rev().enumerate() {
    sum[i+1] = (sum[i] + pow * (c as usize - '0' as usize)) % M;
    pow = (pow * 10) % M;
  }
  let mut popl = vec![0i64; M];
  for &x in &sum {
    popl[x] += 1;
  }
  let mut ans = 0;
  for &p in &popl {
    ans += p * (p - 1) / 2;
  }

  println!("{}", ans);
}

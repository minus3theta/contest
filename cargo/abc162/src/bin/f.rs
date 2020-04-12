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
    a: [i64; n],
  }
  let mut sum = vec![0; n+2];
  for (i, &x) in a.iter().enumerate() {
    sum[i+2] = sum[i] + x;
  }
  if n % 2 == 0 {
    let mut ans = - (1i64 << 60);
    let mut i = 0;
    while i <= n {
      ans = cmp::max(ans, sum[i] + sum[n+1] - sum[i+1]);
      i += 2;
    }
    println!("{}", ans);
  } else {
    let mut ans = - (1i64 << 60);
    let mut right = vec![- (1i64 << 60); n+3];
    let mut i = n;
    loop {
      right[i] = cmp::max(right[i+2], sum[i] + sum[n+1] - sum[i+1]);
      if i < 2 {
        break;
      }
      i -= 2;
    }
    let mut j = 0;
    while j < n {
      ans = cmp::max(ans, right[j+1] - sum[j+1] + sum[j]);
      j += 2;
    }
    println!("{}", ans);
  }
}

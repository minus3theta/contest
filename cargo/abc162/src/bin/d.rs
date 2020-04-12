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
    s: Chars,
  }
  let s: Vec<usize> = s.into_iter().map(|c| match c {
    'R' => 0,
    'G' => 1,
    _ => 2,
  }).collect();
  let mut count = vec![vec![0; 3]; n+1];
  for (i, &c) in s.iter().enumerate() {
    for x in 0 .. 3 {
      count[i+1][x] = count[i][x];
    }
    count[i+1][c] += 1;
  }
  let mut ans = 0i64;
  for j in 0 .. n {
    for k in j+1 .. n {
      if s[j] == s[k] {
        continue;
      }
      let x = 3 - s[j] - s[k];
      ans += count[j][x];
      if let Some(i) = (2 * j).checked_sub(k) {
        if s[i] == x {
          ans -= 1;
        }
      }
    }
  }

  println!("{}", ans);
}

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
    m: usize,
    h: [i64; n],
    e: [(Usize1, Usize1); m],
  }
  let mut adj = vec![vec![]; n];
  for (a, b) in e.into_iter() {
    adj[a].push(b);
    adj[b].push(a);
  }
  let mut ans = 0;
  for i in 0..n {
    if adj[i].iter().all(|&j| h[i] > h[j]) {
      ans += 1;
    }
  }

  println!("{}", ans);
}

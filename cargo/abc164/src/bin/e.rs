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
    m: usize,
    s: usize,
    rail: [(Usize1, Usize1, usize, i64); m],
    ex: [(usize, i64); n],
  }
  let mut edge = vec![vec![]; n];
  for (u, v, a, b) in rail.into_iter() {
    edge[u].push((v, a, b));
    edge[v].push((u, a, b));
  }
  use std::cmp::Reverse;
  use std::collections::BinaryHeap;
  // time, place, silver
  let mut q: BinaryHeap<(Reverse<i64>, usize, usize)> = BinaryHeap::new();
  let max_s = 50 * m * 2;
  let s = cmp::min(s, max_s);
  q.push((Reverse(0), 0, s));
  let mut time = vec![vec![1i64 << 60; max_s + 1]; n];
  time[0][s] = 0;
  let next = |i: usize, x: usize, now: i64, add: i64, q: &mut BinaryHeap<_>, time: &mut Vec<Vec<i64>>| {
    if now + add < time[i][x] {
      time[i][x] = now + add;
      q.push((
        Reverse(time[i][x]),
        i,
        x
      ));
    }
  };
  while let Some((Reverse(t), i, x)) = q.pop() {
    next(i, cmp::min(x + ex[i].0, max_s), t, ex[i].1, &mut q, &mut time);
    for &(j, a, b) in &edge[i] {
      if let Some(y) = x.checked_sub(a) {
        next(j, y, t, b, &mut q, &mut time);
      }
    }
  }
  for i in 1 .. n {
    println!("{}", time[i].iter().min().unwrap());
  }
}

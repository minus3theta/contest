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
    k: i64,
    a: [Usize1; n],
  }
  let mut head = 0;
  let mut cycle = 0;
  let mut visited = vec![None; n];
  let mut current = 0;
  // visited[current] = Some(0);
  let mut index = vec![0; n];
  for i in 0i64.. {
    if let Some(v) = visited[current] {
      head = v;
      cycle = i - v;
      break;
    }
    visited[current] = Some(i);
    index[i as usize] = current;
    current = a[current];
  }
  let ans = if k < head {
    index[k as usize]
  } else {
    index[((k - head) % cycle + head) as usize]
  } + 1;

  println!("{}", ans);
}

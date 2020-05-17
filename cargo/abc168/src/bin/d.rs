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
    es: [(Usize1, Usize1); m],
  }
  let mut adj = vec![vec![]; n];
  for (u, v) in es.into_iter() {
    adj[u].push(v);
    adj[v].push(u);
  }
  use std::collections::VecDeque;
  let mut point = vec![None; n];
  let mut que = VecDeque::new();
  que.push_back(0);
  while let Some(v) = que.pop_front() {
    for &u in &adj[v] {
      if point[u] == None {
        point[u] = Some(v);
        que.push_back(u);
      }
    }
  }

  println!("Yes");
  for p in point.iter().skip(1) {
    println!("{}", p.unwrap() + 1);
  }
}

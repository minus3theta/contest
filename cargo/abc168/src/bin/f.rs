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

use std::collections::{BTreeSet, HashMap};

const INF: i64 = 1 << 30;

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    vs: [(i64, i64, i64); n],
    hs: [(i64, i64, i64); m],
  }
  let mut xs = BTreeSet::new();
  let mut ys = BTreeSet::new();
  xs.insert(0);
  xs.insert(-INF);
  xs.insert(INF);
  ys.insert(0);
  ys.insert(-INF);
  ys.insert(INF);
  for &(a, b, c) in &vs {
    xs.insert(a);
    xs.insert(b);
    ys.insert(c);
  }
  for &(d, e, f) in &hs {
    xs.insert(d);
    ys.insert(e);
    ys.insert(f);
  }
  let x_diff = diffs(&xs);
  let x_cells = x_diff.len();
  let y_diff = diffs(&ys);
  let y_cells = y_diff.len();
  let x_comp: HashMap<_, _> = xs.into_iter().enumerate().map(|(i, x)| (x, i)).collect();
  let y_comp: HashMap<_, _> = ys.into_iter().enumerate().map(|(i, y)| (y, i)).collect();
  // l, r, u, d
  let mut moves = vec![vec![0b1111; y_cells]; x_cells];
  for x in 0..x_cells {
    moves[x][0] &= 0b1110;
    moves[x][y_cells - 1] &= 0b1101;
  }
  for y in 0..y_cells {
    moves[0][y] &= 0b1011;
    moves[x_cells - 1][y] &= 0b0111;
  }
  for &(a, b, c) in &vs {
    let ai = x_comp[&a];
    let bi = x_comp[&b];
    let ci = y_comp[&c];
    for xi in 0..x_cells {
      if ai <= xi && xi < bi {
        moves[xi][ci] &= 0b1110;
        moves[xi][ci - 1] &= 0b1101;
      }
    }
  }
  for &(d, e, f) in &hs {
    let di = x_comp[&d];
    let ei = y_comp[&e];
    let fi = y_comp[&f];
    for yi in 0..y_cells {
      if ei <= yi && yi < fi {
        moves[di][yi] &= 0b1011;
        moves[di - 1][yi] &= 0b0111;
      }
    }
  }

  let mut visited = vec![vec![false; y_cells]; x_cells];
  visited[x_comp[&0]][y_comp[&0]] = true;
  let mut que = Vec::new();
  que.push((x_comp[&0], y_comp[&0]));
  while let Some((x, y)) = que.pop() {
    visited[x][y] = true;
    for (i, &(dx, dy)) in [(0, std::usize::MAX), (0, 1), (std::usize::MAX, 0), (1, 0)]
      .iter()
      .enumerate()
    {
      let (nx, ny) = (x.wrapping_add(dx), y.wrapping_add(dy));
      if nx < x_cells && ny < y_cells && (moves[x][y] >> i & 1 != 0) {
        if !visited[nx][ny] {
          que.push((nx, ny));
        }
      }
    }
  }

  if visited[0][0] {
    println!("INF");
  } else {
    let mut ans = 0;
    for x in 0..x_cells {
      for y in 0..y_cells {
        if visited[x][y] {
          ans += x_diff[x] * y_diff[y];
        }
      }
    }
    println!("{}", ans);
  }
}

fn diffs(xs: &BTreeSet<i64>) -> Vec<i64> {
  xs.iter()
    .cloned()
    .tuple_windows::<_>()
    .map(|(a, b)| b - a)
    .collect()
}

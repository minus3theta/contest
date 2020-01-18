#[allow(unused_imports)]
use std::cmp;
use std::io::Write;

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
    let mut iter = $s.split_whitespace();
    input_inner!{iter, $($r)*}
  };
  ($($r:tt)*) => {
    let s = {
      use std::io::Read;
      let mut s = String::new();
      std::io::stdin().read_to_string(&mut s).unwrap();
      s
    };
    let mut iter = s.split_whitespace();
    input_inner!{iter, $($r)*}
  };
}

macro_rules! input_inner {
  ($iter:expr) => {};
  ($iter:expr, ) => {};

  ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
    let $var = read_value!($iter, $t);
    input_inner!{$iter $($r)*}
  };
}

macro_rules! read_value {
  ($iter:expr, ( $($t:tt),* )) => {
    ( $(read_value!($iter, $t)),* )
  };

  ($iter:expr, [ $t:tt ; $len:expr ]) => {
    (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
  };

  ($iter:expr, chars) => {
    read_value!($iter, String).chars().collect::<Vec<char>>()
  };

  ($iter:expr, usize1) => {
    read_value!($iter, usize) - 1
  };

  ($iter:expr, [ $t:tt ]) => {{
    let len = read_value!($iter, usize);
    (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
  }};

  ($iter:expr, $t:ty) => {
    $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

use std::collections::VecDeque;

fn bfs(adj: &Vec<Vec<usize>>, dist: &mut Vec<i64>, start: usize) {
  dist[start] = 0;
  let mut q = VecDeque::new();
  q.push_back(start);
  while let Some(v) = q.pop_front() {
    let d = dist[v];
    for &next in &adj[v] {
      if dist[next] < d {
        continue;
      }
      dist[next] = d + 1;
      q.push_back(next);
    }
  }
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    u: usize1,
    v: usize1,
    es: [(usize1, usize1); n-1],
  }
  let mut adj = vec![vec![]; n];
  for &(a, b) in &es {
    adj[a].push(b);
    adj[b].push(a);
  }
  let mut adist = vec![1i64 << 60; n];
  bfs(&adj, &mut adist, v);
  let mut tdist = vec![1i64 << 60; n];
  bfs(&adj, &mut tdist, u);
  let mut max_ad = 0;
  for i in 0 .. n {
    if tdist[i] <= adist[i] {
      max_ad = cmp::max(max_ad, adist[i]);
    }
  }

  puts!("{}\n", max_ad - 1);
}

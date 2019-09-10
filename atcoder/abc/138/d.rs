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

  ($iter:expr, $t:ty) => {
    $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

fn dfs(v: usize, par: i64, adj: &Vec<Vec<usize>>, vq: &Vec<i64>, count: &mut Vec<i64>) {
  if count[v] >= 0 {
    return;
  }
  count[v] = par + vq[v];
  for &u in &adj[v] {
    dfs(u, count[v], adj, vq, count);
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
    q: usize,
    edge: [(usize1, usize1); n-1],
    query: [(usize1, i64); q],
  }
  let mut adj = vec![vec![]; n];
  for &(a, b) in &edge {
    adj[a].push(b);
    adj[b].push(a);
  }
  let mut vq = vec![0i64; n];
  for &(p, x) in &query {
    vq[p] += x;
  }
  let mut count = vec![-1i64; n];
  dfs(0, 0, &adj, &vq, &mut count);
  for &c in &count {
    puts!("{} ", c)
  }
  puts!("\n");
}

#[allow(unused_imports)]
use std::cmp;

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
    let mut iter = $s.split_whitespace();
    input_inner!{iter, $($r)*}
  };
  ($($r:tt)*) => {
    #[allow(unused_mut)]
    let mut s = {
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

fn bottom_up(vs: &Vec<Vec<usize>>, sub_pattern: &mut Vec<i64>, children: &mut Vec<Vec<(usize, i64)>>, v: usize, m: i64) -> i64 {
  let mut pat = 1;
  sub_pattern[v] = 1;
  for &u in vs[v].iter() {
    if sub_pattern[u] >= 0 {
      continue;
    }
    let p = bottom_up(vs, sub_pattern, children, u, m);
    pat = (pat * (p + 1)) % m;
    children[v].push((u, p));
  }
  sub_pattern[v] = pat;
  pat
}

fn top_down(sub_pattern: &Vec<i64>, children: &Vec<Vec<(usize, i64)>>, pattern: &mut Vec<i64>, par_pattern: i64, v: usize, m: i64) {
  pattern[v] = (par_pattern + 1) * sub_pattern[v] % m;
  let mut left_scan = vec![1i64; children[v].len() + 1];
  for (i, &(_, p)) in children[v].iter().enumerate() {
    left_scan[i+1] = left_scan[i] * (p + 1) % m;
  }
  let mut right_scan = vec![1i64; children[v].len() + 1];
  for (i, &(_, p)) in children[v].iter().enumerate().rev() {
    right_scan[i] = right_scan[i+1] * (p + 1) % m;
  }
  for (i, &(u, _)) in children[v].iter().enumerate() {
    let pp = left_scan[i] * right_scan[i+1] % m * (par_pattern + 1) % m;
    top_down(sub_pattern, children, pattern, pp, u, m);
  }
}

fn main() {
  input! {
    n: usize,
    m: i64,
    xys: [(usize1, usize1); n-1]
  }
  let mut vs = vec![vec![]; n];
  for &(x, y) in xys.iter() {
    vs[x].push(y);
    vs[y].push(x);
  }
  let mut sub_pattern = vec![-1i64; n];
  let mut children = vec![vec![]; n];
  bottom_up(&vs, &mut sub_pattern, &mut children, 0, m);
  let mut pattern = vec![0i64; n];
  top_down(&sub_pattern, &children, &mut pattern, 0, 0, m);
  for &p in pattern.iter() {
    println!("{}", p);
  }
}

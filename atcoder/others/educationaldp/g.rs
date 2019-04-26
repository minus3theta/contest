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

fn top_sort(point: usize, adjs: &Vec<Vec<usize>>, visited: &mut Vec<bool>, sorted: &mut Vec<usize>) {
  if visited[point] {
    return;
  }
  visited[point] = true;
  for &next in adjs[point].iter() {
    top_sort(next, adjs, visited, sorted);
  }
  sorted.push(point);
}

fn main() {
  input! {
    n: usize,
    m: usize,
    edges: [(usize1, usize1); m]
  }
  let mut adjs = vec![vec![]; n];
  for &(src, dst) in edges.iter() {
    adjs[src].push(dst);
  }
  let mut visited = vec![false; n];
  let mut sorted = Vec::new();
  for point in 0..n {
    top_sort(point, &adjs, &mut visited, &mut sorted);
  }
  let mut path_len = vec![0; n];
  for &point in sorted.iter() {
    for &next in adjs[point].iter() {
      path_len[point] = cmp::max(path_len[point], path_len[next] + 1);
    }
  }
  let ans = path_len.iter().fold(0, |current, val| cmp::max(current, *val));
  println!("{}", ans);
}

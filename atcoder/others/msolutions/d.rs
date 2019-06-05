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

fn write(v: usize, vs: &Vec<Vec<usize>>, nums: &mut Vec<i64>, cs: &Vec<i64>, mut ci: usize) -> usize {
  nums[v] = cs[ci];
  for &u in vs[v].iter() {
    if nums[u] <= 0 {
      ci = write(u, vs, nums, cs, ci - 1);
    }
  }
  ci
}

fn main() {
  input! {
    n: usize,
    edges: [(usize1, usize1); n - 1],
    cs: [i64; n],
  }
  let mut vs = vec![vec![]; n];
  for &(a, b) in edges.iter() {
    vs[a].push(b);
    vs[b].push(a);
  }
  let mut cs = cs;
  cs.sort();
  let mut nums = vec![0; n];
  write(0, &vs, &mut nums, &cs, n - 1);
  let sum = cs.iter().sum::<i64>() - cs[n-1];

  println!("{}", sum);
  for &x in nums.iter() {
    print!("{} ", x);
  }
  println!("");
}

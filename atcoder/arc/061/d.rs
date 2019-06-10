#[allow(unused_imports)]
use std::cmp;
use std::collections::BTreeMap;

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

fn main() {
  input! {
    h: i64,
    w: i64,
    n: usize,
    black: [(i64, i64); n],
  }
  let mut map = BTreeMap::new();
  for &(i, j) in &black {
    let i = i - 1;
    let j = j - 1;
    for ti in cmp::max(1, i-1) .. cmp::min(h-1, i+2) {
      for tj in cmp::max(1, j-1) .. cmp::min(w-1, j+2) {
        if let Some(current) = map.get_mut(&(ti, tj)) {
          *current += 1;
          continue;
        }
        map.insert((ti, tj), 1);
      }
    }
  }
  let mut ans = vec![0i64; 10];
  for &v in map.values() {
    ans[v] += 1;
  }
  let sum: i64 = ans.iter().sum();
  ans[0] = (h - 2) * (w - 2) - sum;
  for &a in &ans {
    println!("{}", a);
  }
}

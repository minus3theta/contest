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

fn main() {
  input! {
    n: usize,
    c: usize,
    d: [[i64; c]; c],
    field: [[usize1; n]; n],
  }
  let mut pop = vec![vec![0; 3]; c];
  for i in 0 .. n {
    for j in 0 .. n {
      pop[field[i][j]][(i + j) % 3] += 1;
    }
  }
  let mut ans = 1i64 << 60;
  for c0 in 0 .. c {
    for c1 in 0 .. c {
      if c1 == c0 {
        continue;
      }
      for c2 in 0 .. c {
        if c2 == c0 || c2 == c1 {
          continue;
        }
        let mut cost = 0;
        for old_color in 0 .. c {
          cost += d[old_color][c0] * pop[old_color][0] + d[old_color][c1] * pop[old_color][1] + d[old_color][c2] * pop[old_color][2];
        }
        ans = cmp::min(ans, cost)
      }
    }
  }

  println!("{}", ans);
}

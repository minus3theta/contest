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

use std::collections::HashSet;

fn gcd(x: i64, y: i64) -> i64 {
  if y == 0 {
    x
  } else {
    gcd(y, x % y)
  }
}

fn solve(n: usize, s: Vec<char>, m: usize, x: usize, y: usize) -> i64 {
  let mut dp = vec![vec![HashSet::new(); y + 2]; n + 1];
  dp[0][0].insert(0);
  for i in 0 .. n {
    for sep in 0 .. y + 1 {
      for prev in 0 .. m {
        // i - prev < 0
        if prev > i {
          break;
        }
        let mut num = 0;
        for d in i - prev .. i + 1 {
          num *= 10;
          num += s[d] as i64 - '0' as i64;
        }
        let vals = dp[i-prev][sep].clone();
        // println!("{:?} * {}", &vals, num);
        for &val in &vals {
          dp[i+1][sep+1].insert(gcd(val, num));
        }
        // println!("{} -> {} ({}): {:?}", prev, i, sep, &dp[i+1][sep+1]);
      }
    }
  }
  // println!("{:?}", &dp);

  let mut ans = -1;
  for sep in x .. y + 1 {
    for &g in &dp[n][sep+1] {
      ans = cmp::max(ans, g);
    }
  }
  ans
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    t: usize,
    cases: [(usize, chars, usize, usize, usize); t],
  }
  for (n, s, m, x, y) in cases.into_iter() {
    puts!("{}\n", solve(n, s, m, x, y));
  }
}

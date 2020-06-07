#[allow(unused_imports)]
use std::cmp;
use std::io::Write;

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
#[allow(unused_macros)]
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

enum Info {
  Time(i64),
  Before(i64),
}

use Info::{Time, Before};

impl Info {
  fn new(x: i64) -> Info {
    if x > 0 {
      Time(x)
    } else {
      Before(-x)
    }
  }
}

fn solve(_c: usize, d: usize, x: Vec<i64>, es: Vec<(usize, usize)>) -> Vec<i64> {
  let info: Vec<_> = Some(0).into_iter().chain(x.into_iter()).map(Info::new).collect();
  let mut ans = vec![999999; d];
  for (i, &(u, v)) in es.iter().enumerate() {
    match (&info[u], &info[v]) {
      (&Before(bu), &Before(bv)) => {
        let diff = (bu - bv).abs();
        if diff != 0 {
          ans[i] = diff;
        }
      }
      _ => unreachable!()
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
  let s = {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s
  };
  let mut iter = s.split_whitespace();
  input_inner! {
    iter,
    t: usize,
  }
  for case in 0..t {
    input_inner! {
      iter,
      c: usize,
      d: usize,
      x: [i64; c-1],
      es: [(usize1, usize1); d],
    }
    puts!("Case #{}:", case+1);
    for &latency in &solve(c, d, x, es) {
      puts!(" {}", latency);
    }
    puts!("\n");
  }
}

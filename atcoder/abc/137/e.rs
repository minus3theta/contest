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

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    m: usize,
    p: i64,
    es: [(usize1, usize1, i64); m],
  }
  let es: Vec<_> = es.into_iter().map(|(a, b, c)| (a, b, c - p)).collect();
  let mut score = vec![-(1i64 << 60); n];
  score[0] = 0;
  let mut reach_start = vec![false; n];
  reach_start[0] = true;
  let mut reach_end = vec![false; n];
  reach_end[n-1] = true;
  for _ in 0 .. n - 1 {
    for &(a, b, c) in &es {
      score[b] = cmp::max(score[b], score[a] + c);
      reach_start[b] = reach_start[b] || reach_start[a];
      reach_end[a] = reach_end[a] || reach_end[b];
    }
  }
  let mut update = false;
  for &(a, b, c) in &es {
    if reach_start[b] && reach_end[b] && score[a] + c > score[b] {
      update = true;
      break;
    }
  }
  if update {
    puts!("-1\n");
  } else {
    puts!("{}\n", cmp::max(score[n-1], 0));
  }
}

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
    d: usize,
    g: i64,
    ps: [(i64, i64); d],
  }
  let mut ans = 1 << 30;
  for pat in 0 .. 1 << d {
    let mut score = 0;
    let mut solve = 0;
    for (i, &(p, c)) in ps.iter().enumerate() {
      if (pat >> i) & 1 != 0 {
        solve += p;
        score += p * (i as i64 + 1) * 100 + c;
      }
    }
    let rest = g - score;
    if rest > 0 {
      if let Some((i, &(p, _))) = ps.iter().enumerate().rev().find(|&(i, _)| (pat >> i) & 1 == 0) {
        let s = (i as i64 + 1) * 100;
        let n = (rest + s - 1) / s;
        if n >= p {
          continue;
        }
        solve += n;
      }
    }
    ans = cmp::min(ans, solve);
  }

  puts!("{}\n", ans);
}

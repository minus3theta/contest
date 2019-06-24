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

fn minf(x: f64, y: f64) -> f64 {
  use std::cmp::Ordering::*;
  match x.partial_cmp(&y) {
    Some(Less) => x,
    Some(Equal) => x,
    Some(Greater) => y,
    _ => unreachable!(),
  }
}

fn main() {
  input! {
    n: usize,
    ts: [usize; n],
    vs: [f64; n],
  }
  let total: usize = ts.iter().sum();
  let mut ranges = vec![(0.0, 0.0, 0.0)];
  {
    let mut prev = 0;
    for (&t, &v) in ts.iter().zip(&vs) {
      ranges.push((prev as f64, (prev + t) as f64, v));
      prev = prev + t;
    }
  }
  ranges.push((total as f64, total as f64, 0.0));
  let mut v_val = vec![0.0];
  for i in 0 .. 2 * total {
    let x = (i + 1) as f64 / 2.0;
    let mut minv = 1e18;
    for &(l, r, v) in &ranges {
      if x < l {
        minv = minf(minv, v + (l - x));
      } else if x > r {
        minv = minf(minv, v + (x - r));
      } else {
        minv = minf(minv, v);
      }
    }
    v_val.push(minv);
  }
  let mut ans = 0.0;
  for i in 0 .. v_val.len() - 1 {
    ans += (v_val[i] + v_val[i+1]) / 4.0;
  }

  println!("{:.10}", ans);
}

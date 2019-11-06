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

fn cum(v: &Vec<i64>) -> Vec<i64> {
  let mut ret = vec![0; v.len() + 1];
  for (i, &x) in v.iter().enumerate() {
    ret[i+1] = ret[i] + x;
  }
  ret
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    cap: usize,
    wv: [(usize, i64); n],
  }
  let mut item = vec![vec![]; 4];
  let w0 = wv[0].0;
  for &(w, v) in &wv {
    item[w - w0].push(v);
  }
  for vec in item.iter_mut() {
    vec.sort();
    vec.reverse();
  }
  let sum: Vec<_> = item.iter().map(|v| cum(v)).collect();
  let mut ans = 0;
  for i0 in 0 .. sum[0].len() {
    for i1 in 0 .. sum[1].len() {
      for i2 in 0 .. sum[2].len() {
        for i3 in 0 .. sum[3].len() {
          if w0 * i0 + (w0 + 1) * i1 + (w0 + 2) * i2 + (w0 + 3) * i3 <= cap {
            ans = cmp::max(ans, sum[0][i0] + sum[1][i1] + sum[2][i2] + sum[3][i3]);
          }
        }
      }
    }
  }

  puts!("{}\n", ans);
}

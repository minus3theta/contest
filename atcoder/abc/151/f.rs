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

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    ps: [(i32, i32); n],
  }
  let mut ans = 1e100;
  for i in 0 .. n {
    for j in i+1 .. n {
      let dx1 = ps[j].0 - ps[i].0;
      let dy1 = ps[j].1 - ps[i].1;
      for k in j .. n {
        let dx2 = ps[k].0 - ps[i].0;
        let dy2 = ps[k].1 - ps[i].1;
        let center = if dx1 * dy2 == dx2 * dy1 {
          (
            (cmp::max(ps[i].0, cmp::max(ps[j].0, ps[k].0)) + cmp::min(ps[i].0, cmp::min(ps[j].0, ps[k].0))) as f64 / 2.0,
            (cmp::max(ps[i].1, cmp::max(ps[j].1, ps[k].1)) + cmp::min(ps[i].1, cmp::min(ps[j].1, ps[k].1))) as f64 / 2.0
          )
        } else {
          let det = (dx2 * dy1 - dx1 * dy2) as f64;
          let t = (-dx2 * dx1 + dx2 * dx2 - dy1 * dy2 + dy2 * dy2) as f64 / 2.0 / det;
          let dx1 = dx1 as f64;
          let dx2 = dx2 as f64;
          let dy1 = dy1 as f64;
          let dy2 = dy2 as f64;
          (
            (ps[i].0 as f64 + dx1 / 2.0 + t * dy1),
            (ps[i].1 as f64 + dy1 / 2.0 - t * dx1)
          )
        };
        let mut max = 0.0;
        for l in 0 .. n {
          let d = ((ps[l].0 as f64 - center.0).powi(2) + (ps[l].1 as f64 - center.1).powi(2)).sqrt();
          if d > max {
            max = d;
          }
        }
        if max < ans {
          ans = max;
        }
      }
    }
  }

  puts!("{:.15}\n", ans);
}

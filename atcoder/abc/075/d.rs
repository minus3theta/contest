#[allow(unused_imports)]
use std::cmp;

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
  input! {
    n: usize,
    k: usize,
    points: [(i64, i64); n],
  }
  let mut points = points;
  points.sort();
  let mut ans = 1i64 << 62;
  for left in 0 .. n {
    let minx = points[left].0;
    for top in 0 .. n {
      for bottom in 0 .. n {
        let miny = points[bottom].1;
        let maxy = points[top].1;
        if miny > maxy {
          continue;
        }
        let mut count = 0;
        for p in left .. n {
          let p = points[p];
          if miny <= p.1 && p.1 <= maxy {
            count += 1;
            if count >= k {
              let maxx = p.0;
              ans = cmp::min(ans, (maxx - minx) * (maxy - miny));
            }
          }
        }
      }
    }
  }

  println!("{}", ans);
}

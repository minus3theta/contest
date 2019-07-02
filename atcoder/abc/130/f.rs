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

fn add_changes(inc: Option<i64>, dec: Option<i64>, same: Option<i64>, changes: &mut Vec<f64>) {
  if let (Some(inc), Some(dec)) = (inc, dec) {
    // inc + t = dec - t
    let t = (dec - inc) as f64 / 2.0;
    if t > 0.0 {
      changes.push(t);
    }
  }
  if let (Some(inc), Some(same)) = (inc, same) {
    // inc + t = same
    let t = (same - inc) as f64;
    if t > 0.0 {
      changes.push(t);
    }
  }
  if let (Some(dec), Some(same)) = (dec, same) {
    // dec - t = same
    let t = (dec - same) as f64;
    if t > 0.0 {
      changes.push(t);
    }
  }
}

fn main() {
  input! {
    n: usize,
    point: [(i64, i64, chars); n],
  }
  let ls: Vec<_> = point.iter().filter(|&&(_, _, ref s)| s[0] == 'L').map(|&(x, y, _)| (x, y)).collect();
  let rs: Vec<_> = point.iter().filter(|&&(_, _, ref s)| s[0] == 'R').map(|&(x, y, _)| (x, y)).collect();
  let us: Vec<_> = point.iter().filter(|&&(_, _, ref s)| s[0] == 'U').map(|&(x, y, _)| (x, y)).collect();
  let ds: Vec<_> = point.iter().filter(|&&(_, _, ref s)| s[0] == 'D').map(|&(x, y, _)| (x, y)).collect();
  let mut changes = vec![0.0];
  add_changes(rs.iter().map(|&(x, _)| x).min(), ls.iter().map(|&(x, _)| x).min(), us.iter().chain(ds.iter()).map(|&(x, _)| x).min(), &mut changes);
  add_changes(rs.iter().map(|&(x, _)| x).max(), ls.iter().map(|&(x, _)| x).max(), us.iter().chain(ds.iter()).map(|&(x, _)| x).max(), &mut changes);
  add_changes(us.iter().map(|&(_, y)| y).min(), ds.iter().map(|&(_, y)| y).min(), ls.iter().chain(rs.iter()).map(|&(_, y)| y).min(), &mut changes);
  add_changes(us.iter().map(|&(_, y)| y).max(), ds.iter().map(|&(_, y)| y).max(), ls.iter().chain(rs.iter()).map(|&(_, y)| y).max(), &mut changes);
  let mut ans = 1e30;
  for &t in &changes {
    let mut x_max = -1e30;
    let mut x_min = 1e30;
    let mut y_max = -1e30;
    let mut y_min = 1e30;
    {
      let mut update = |x, y| {
        f_max(&mut x_max, x);
        f_min(&mut x_min, x);
        f_max(&mut y_max, y);
        f_min(&mut y_min, y);
      };
      for &(x, y) in &ls {
        update(x as f64 - t, y as f64);
      }
      for &(x, y) in &rs {
        update(x as f64 + t, y as f64);
      }
      for &(x, y) in &us {
        update(x as f64, y as f64 + t);
      }
      for &(x, y) in &ds {
        update(x as f64, y as f64 - t);
      }
    }
    f_min(&mut ans, (x_max - x_min) * (y_max - y_min));
  }

  println!("{:.15}", ans);
}

fn f_max(target: &mut f64, other: f64) {
  if other > *target {
    *target = other;
  }
}

fn f_min(target: &mut f64, other: f64) {
  if other < *target {
    *target = other;
  }
}

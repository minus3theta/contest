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

fn gcd(x: i64, y: i64) -> i64 {
  if x == 0 {
    y
  } else {
    gcd(y % x, x)
  }
}

fn main() {
  input! {
    n: usize,
    a: [i64; n],
  }
  let mut scan_left = vec![0; n+1];
  let mut scan_right = vec![0; n+1];
  for i in 0 .. n {
    scan_left[i+1] = gcd(scan_left[i], a[i]);
  }
  for i in (0 .. n).rev() {
    scan_right[i] = gcd(scan_right[i+1], a[i]);
  }
  let mut max = 0;
  for i in 0 .. n {
    max = cmp::max(max, gcd(scan_left[i], scan_right[i+1]));
  }

  println!("{}", max);
}

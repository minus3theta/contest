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

fn solve_one_dim(l0: usize, l1: usize) -> Vec<i64> {
  let s: Vec<_> = (0 .. l0 + 1).map(|i| {
    if i % l1 == 0 {
      - ((i / l1) as i64)
    } else {
      1000 - ((i / l1) as i64)
    }
  }).collect();
  let mut a = vec![];
  for i in 0 .. l0 {
    a.push(s[i+1] - s[i]);
  }
  a
}

fn main() {
  input! {
    h0: usize,
    w0: usize,
    h1: usize,
    w1: usize,
  }
  if h0 % h1 != 0 {
    println!("Yes");
    let a = solve_one_dim(h0, h1);
    for &x in a.iter() {
      for _ in 0 .. w0 {
        print!("{} ", x);
      }
      println!("");
    }
  } else if w0 % w1 != 0 {
    println!("Yes");
    let a = solve_one_dim(w0, w1);
    for _ in 0 .. h0 {
      for x in a.iter() {
        print!("{} ", x);
      }
      println!("");
    }
  } else {
    println!("No");
  }
}

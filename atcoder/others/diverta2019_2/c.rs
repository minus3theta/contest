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

fn solve(pos: &mut Vec<i64>, neg: &mut Vec<i64>) -> i64 {
  if pos.len() == 1 && neg.len() == 1 {
    let n = neg[0];
    let p = pos[0];
    println!("{} {}", p, n);
    p - n
  } else if pos.len() >= neg.len() {
    let p = pos.pop().unwrap();
    let x = solve(neg, pos);
    println!("{} {}", p, x);
    p - x
  } else {
    let n = neg.pop().unwrap();
    let x = solve(pos, neg);
    println!("{} {}", x, n);
    x - n
  }
}

fn main() {
  input! {
    n: usize,
    aa: [i64; n],
  }
  let mut aa = aa;
  aa.sort();
  let sum: i64 = aa.iter().sum();
  let val;
  let mut pos;
  let mut neg;
  if *aa.last().unwrap() < 0 {
    val = - sum + 2 * *aa.last().unwrap();
    pos = vec![*aa.last().unwrap()];
    neg = aa.iter().cloned().take(n-1).collect();
  } else if *aa.first().unwrap() > 0 {
    val = sum - 2 * *aa.first().unwrap();
    pos = aa.iter().cloned().skip(1).collect();
    neg = vec![*aa.first().unwrap()];
  } else {
    val = aa.iter().map(|&x| x.abs()).sum();
    pos = aa.iter().cloned().filter(|&x| x > 0).collect();
    neg = aa.iter().cloned().filter(|&x| x <= 0).collect();
    if pos.len() == 0 {
      let x = neg.pop().unwrap();
      pos.push(x);
    }
  }
  println!("{}", val);
  let stack_size = 104_857_600;
  let thd = std::thread::Builder::new().stack_size(stack_size);
  thd.spawn(move || solve(&mut pos, &mut neg)).unwrap().join().unwrap();
}

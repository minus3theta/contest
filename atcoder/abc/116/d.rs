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

fn main() {
  input! {
    n: usize,
    k: usize,
    sushi: [(usize1, i64); n],
  }
  let mut sushi = sushi;
  sushi.sort_by(|&(_, d1), &(_, d2)| d2.cmp(&d1));
  let mut kind_used = vec![false; n];
  let mut point_base = 0;
  let mut remove_candidate = vec![];
  let mut kind: i64 = 0;
  for &(t, d) in sushi.iter().take(k) {
    if kind_used[t] {
      remove_candidate.push(d);
    } else {
      kind_used[t] = true;
      kind += 1;
    }
    point_base += d;
  }
  remove_candidate.sort();
  let mut max_point = point_base + kind * kind;
  let mut add_candidate = vec![];
  for &(t, d) in sushi.iter().skip(k) {
    if ! kind_used[t] {
      add_candidate.push(d);
      kind_used[t] = true;
    }
  }
  for (i, &d) in add_candidate.iter().enumerate() {
    if i >= remove_candidate.len() {
      break;
    }
    point_base += d - remove_candidate[i];
    kind += 1;
    max_point = cmp::max(max_point, point_base + kind * kind);
  }

  println!("{}", max_point);
}

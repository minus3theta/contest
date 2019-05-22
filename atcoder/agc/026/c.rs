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
  use std::collections::HashMap;
  input! {
    n: usize,
    s: chars,
  }
  let (front, back) = s.split_at(n);
  let back: Vec<_> = back.into_iter().rev().collect();
  let mut hash = HashMap::new();
  for mask in 0 .. 1 << n {
    let mut rb = (String::new(), String::new());
    for (i, &c) in front.iter().enumerate() {
      if (mask >> i) & 1 == 1 {
        rb.1.push(c);
      } else {
        rb.0.push(c);
      }
    }
    if let Some(count) = hash.get_mut(&rb) {
      *count += 1i64;
      continue;
    }
    hash.insert(rb.clone(), 1);
  }
  let mut ans = 0i64;
  for mask in 0 .. 1 << n {
    let mut rb = (String::new(), String::new());
    for (i, &c) in back.iter().enumerate() {
      if (mask >> i) & 1 == 1 {
        rb.1.push(*c);
      } else {
        rb.0.push(*c);
      }
    }
    if let Some(count) = hash.get(&rb) {
      ans += *count;
    }
  }
  println!("{}", ans);
}

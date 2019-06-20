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

const SIZE: usize = 100_010;

fn main() {
  input! {
    q: usize,
    query: [(usize, usize); q],
  }
  let mut is_prime = vec![false; SIZE];
  for i in 2 .. SIZE {
    is_prime[i] = true;
    for j in (2 .. SIZE).take_while(|j| j * j <= i) {
      if i % j == 0 {
        is_prime[i] = false;
        break;
      }
    }
  }
  let mut like = vec![0i64; SIZE];
  for i in 3 .. SIZE {
    if i % 2 == 1 && is_prime[i] && is_prime[(i + 1) / 2] {
      like[i] = 1;
    }
  }
  for i in 1 .. SIZE {
    like[i] += like[i - 1];
  }
  for &(l, r) in &query {
    println!("{}", like[r] - like[l - 1]);
  }
}

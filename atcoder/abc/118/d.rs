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
    m: usize,
    a: [usize; m],
  }
  let mut a = a;
  a.sort();
  let digit_cost = vec![0, 2, 5, 5, 4, 5, 6, 3, 7, 6];
  let mut dp = vec![-(1i32 << 30); n+1];
  dp[0] = 0;
  for used_match in 1 .. n + 1 {
    for &digit in a.iter() {
      let cost = digit_cost[digit];
      if let Some(prev_used) = used_match.checked_sub(cost) {
        dp[used_match] = cmp::max(dp[used_match], dp[prev_used] + 1);
      }
    }
  }
  let mut ans = String::new();
  let mut used_match = n;
  for _ in 0 .. dp[n] {
    for &digit in a.iter().rev() {
      let cost = digit_cost[digit];
      if let Some(prev_used) = used_match.checked_sub(cost) {
        if dp[prev_used] + 1 == dp[used_match] {
          ans.push(std::char::from_digit(digit as u32, 10).unwrap());
          used_match = prev_used;
          break;
        }
      }
    }
  }

  println!("{}", ans);
}

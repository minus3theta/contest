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

const MODULO: i64 = 1_000_000_007;

fn main() {
  input! {
    n: usize,
    k: usize,
    aa: [usize; n]
  }
  let mut dp = vec![vec![0i64; k+1]; 2];
  dp[0][0] = 1;
  for (i, &a) in aa.iter().enumerate() {
    dp[(i+1)%2][0] = 1;
    for c in 0 .. k {
      let x = dp[(i+1)%2][c] + dp[i%2][c+1];
      dp[(i+1)%2][c+1] = if x >= MODULO { x - MODULO } else { x };
      if let Some(d) = c.checked_sub(a) {
        let y = dp[(i+1)%2][c+1] - dp[i%2][d];
        dp[(i+1)%2][c+1] = if y < 0 { y + MODULO } else { y };
      }
    }
  }
  println!("{}", dp[n%2][k]);
}

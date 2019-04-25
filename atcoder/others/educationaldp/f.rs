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
  input!{
    s: chars,
    t: chars
  }
  let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];
  for (si, sc) in s.iter().enumerate() {
    for (ti, tc) in t.iter().enumerate() {
      dp[si + 1][ti + 1] = cmp::max(
        dp[si][ti] + if sc == tc {1} else {0},
        cmp::max(dp[si + 1][ti], dp[si][ti + 1])
      );
    }
  }
  let mut lcs = Vec::new();
  let (mut si, mut ti) = (s.len(), t.len());
  while (si, ti) != (0, 0) {
    let cur = dp[si][ti];
    if si >= 1 && ti >= 1 && s[si - 1] == t[ti - 1] {
      let prev = dp[si - 1][ti - 1];
      if prev + 1 == cur {
        lcs.push(s[si - 1]);
        si -= 1;
        ti -= 1;
        continue;
      } else if prev == cur {
        si -= 1;
        ti -= 1;
        continue;
      }
    }
    if si >= 1 && dp[si - 1][ti] == cur {
      si -= 1;
      continue;
    }
    if ti >= 1 && dp[si][ti - 1] == cur {
      ti -= 1;
      continue;
    }
  }
  let lcs: String = lcs.iter().rev().map(|&c| c).collect();
  println!("{}", lcs);
}

#[allow(unused_imports)]
use std::cmp;
use std::io::Write;

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

  ($iter:expr, [ $t:tt ]) => {{
    let len = read_value!($iter, usize);
    (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
  }};

  ($iter:expr, $t:ty) => {
    $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    h: usize,
    w: usize,
    a: [[i32; w]; h],
    b: [[i32; w]; h],
  }
  let diff: Vec<Vec<_>> = a.iter().zip(b.iter()).map(|(ai, bi)| ai.iter().zip(bi.iter()).map(|(x, y)| (x - y).abs()).collect()).collect();
  let val = 80 * (h + w);
  let mut dp = vec![vec![vec![false; val]; w]; h];
  dp[0][0][diff[0][0] as usize] = true;
  for i in 0 .. h {
    for j in 0 .. w {
      for v in 0 .. val {
        let vpos = v + diff[i][j] as usize;
        let vneg = (v as i32 - diff[i][j]).abs() as usize;
        if let Some(pi) = i.checked_sub(1) {
          if vpos < val && dp[pi][j][vpos] || vneg < val && dp[pi][j][vneg] {
            dp[i][j][v] = true;
          }
        }
        if let Some(pj) = j.checked_sub(1) {
          if vpos < val && dp[i][pj][vpos] || vneg < val && dp[i][pj][vneg] {
            dp[i][j][v] = true;
          }
        }
      }
    }
  }
  let ans = dp[h-1][w-1].iter().enumerate().filter(|&(_, &f)| f).next().unwrap().0;

  puts!("{}\n", ans);
}

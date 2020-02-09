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
    n: chars,
    k: usize,
  }
  let n: Vec<_> = n.into_iter().map(|c| c as i64 - '0' as i64).collect();
  let mut same = vec![vec![0; k+1]; n.len()+1];
  let mut less = vec![vec![0; k+1]; n.len()+1];
  same[0][0] = 1;
  for i in 0 .. n.len() {
    for c in 0 .. k + 1 {
      if n[i] == 0 {
        same[i+1][c] += same[i][c];
      }
      if c > 0 && n[i] != 0 {
        same[i+1][c] += same[i][c-1];
      }
      less[i+1][c] = less[i][c];
      if n[i] != 0 {
        less[i+1][c] += same[i][c];
      }
      if c > 0 {
        less[i+1][c] += less[i][c-1] * 9;
        if n[i] > 1 {
          less[i+1][c] += same[i][c-1] * (n[i] - 1);
        }
      }
    }
  }
  // dbg!(&same);
  // dbg!(&less);
  puts!("{}\n", same[n.len()][k] + less[n.len()][k]);
}

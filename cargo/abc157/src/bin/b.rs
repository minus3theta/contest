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
    a: [[i32; 3]; 3],
    n: usize,
    b: [i32; n],
  }
  let mut fld = vec![vec![false; 3]; 3];
  for &x in &b {
    for i in 0 .. 3 {
      for j in 0 .. 3 {
        if a[i][j] == x {
          fld[i][j] = true;
        }
      }
    }
  }
  let mut ans = false;
  for i in 0 .. 3 {
    let mut bingo = true;
    for j in 0 .. 3 {
      bingo = bingo && fld[i][j];
    }
    ans = ans || bingo;
  }
  for j in 0 .. 3 {
    let mut bingo = true;
    for i in 0 .. 3 {
      bingo = bingo && fld[i][j];
    }
    ans = ans || bingo;
  }
  ans = ans || (fld[0][0] && fld[1][1] && fld[2][2]) || (fld[0][2] && fld[1][1] && fld[2][0]);

  puts!("{}\n", if ans { "Yes" } else { "No" });
}

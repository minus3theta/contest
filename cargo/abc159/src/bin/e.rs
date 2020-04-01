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
    k: usize,
    s: [chars; h],
  }
  let mut sum = vec![vec![0usize; w+1]; h+1];
  for i in 0 .. h {
    for j in 0 .. w {
      sum[i+1][j+1] = sum[i][j+1] + sum[i+1][j] - sum[i][j] +
        if s[i][j] == '1' {1} else {0};
    }
  }
  let white = |u: usize, d: usize, l: usize, r: usize| {
    sum[d][r] + sum[u][l] - sum[d][l] - sum[u][r]
  };
  let mut ans = 1_000_000_000;
  'outer: for pat in 0 .. 1 << (h - 1) {
    let find_right = |left| {
      let mut right = left + 1;
      while right <= w {
        let mut up = 0;
        let mut down = 1;
        while up < h {
          if down == h || (pat >> (down - 1)) & 1 != 0 {
            if white(up, down, left, right) > k {
              return right - 1;
            }
            up = down;
          }
          down += 1;
        }
        right += 1;
      }
      right
    };
    let mut cut = 0;
    for c in 0 .. h {
      if (pat >> c) & 1 != 0 {
        cut += 1;
      }
    }
    let mut left = 0;
    while left < w {
      let right = find_right(left);
      if left == right {
        continue 'outer;
      }
      left = right;
      cut += 1;
    }
    ans = cmp::min(ans, cut-1);
  }

  puts!("{}\n", ans);
}

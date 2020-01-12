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

const INF: i64 = 1 << 40;

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    h: usize,
    w: usize,
    fld: [chars; h],
  }
  let mut adj = vec![vec![INF; h * w]; h * w];
  for i in 0 .. h * w {
    adj[i][i] = 0;
  }
  for i in 0 .. h {
    for j in 0 .. w - 1 {
      if fld[i][j] == '.' && fld[i][j+1] == '.' {
        adj[i*w+j][i*w+j+1] = 1;
        adj[i*w+j+1][i*w+j] = 1;
      }
    }
  }
  for i in 0 .. h - 1 {
    for j in 0 .. w {
      if fld[i][j] == '.' && fld[i+1][j] == '.' {
        adj[i*w+j][(i+1)*w+j] = 1;
        adj[(i+1)*w+j][i*w+j] = 1;
      }
    }
  }
  for k in 0 .. h * w {
    for i in 0 .. h * w {
      for j in 0 .. h * w {
        adj[i][j] = cmp::min(adj[i][j], adj[i][k] + adj[k][j]);
      }
    }
  }
  let mut ans = 0;
  for i in 0 .. h * w {
    for j in 0 .. h * w {
      if adj[i][j] != INF {
        ans = cmp::max(ans, adj[i][j]);
      }
    }
  }

  puts!("{}\n", ans);
}

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

const INF: i64 = 1 << 50;

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    m: usize,
    l: i64,
    es: [(usize1, usize1, i64); m],
    q: usize,
    qs: [(usize1, usize1); q],
  }
  let mut dist = vec![vec![INF; n]; n];
  for i in 0 .. n {
    dist[i][i] = 0;
  }
  for &(a, b, c) in &es {
    dist[a][b] = c;
    dist[b][a] = c;
  }
  for k in 0 .. n {
    for i in 0 .. n {
      for j in 0 .. n {
        dist[i][j] = cmp::min(dist[i][j], dist[i][k] + dist[k][j]);
      }
    }
  }
  let mut times = vec![vec![INF; n]; n];
  for i in 0 .. n {
    for j in 0 .. n {
      if dist[i][j] <= l {
        times[i][j] = 1;
      }
    }
  }
  for k in 0 .. n {
    for i in 0 .. n {
      for j in 0 .. n {
        times[i][j] = cmp::min(times[i][j], times[i][k] + times[k][j]);
      }
    }
  }
  for &(s, t) in &qs {
    if times[s][t] < INF {
      puts!("{}\n", times[s][t] - 1);
    } else {
      puts!("-1\n");
    }
  }
}

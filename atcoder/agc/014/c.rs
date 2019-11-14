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
    k: i32,
    field: [chars; h],
  }
  let mut start = (0, 0);
  'outer: for (i, r) in field.iter().enumerate() {
    for (j, &c) in r.iter().enumerate() {
      if c == 'S' {
        start = (i, j);
        break 'outer;
      }
    }
  }
  let mut dist = vec![vec![1 << 30; w]; h];
  dist[start.0][start.1] = 0;
  use std::collections::VecDeque;
  let mut que = VecDeque::new();
  que.push_back(start);
  let dis = [0, 1, 0, -1];
  let djs = [1, 0, -1, 0];
  while let Some((i, j)) = que.pop_front() {
    let d = dist[i][j];
    for (&di, &dj) in dis.iter().zip(djs.iter()) {
      let ni = i as i32 + di;
      let nj = j as i32 + dj;
      if ni < 0 || ni >= h as i32 || nj < 0 || nj >= w as i32 {
        continue;
      }
      let ni = ni as usize;
      let nj = nj as usize;
      if field[ni][nj] == '#' {
        continue;
      }
      if dist[ni][nj] > d + 1 {
        dist[ni][nj] = d + 1;
        que.push_back((ni, nj));
      }
    }
  }
  let mut d_edge = 1 << 30;
  for (i, r) in dist.iter().enumerate() {
    for (j, &d) in r.iter().enumerate() {
      if d <= k {
        d_edge = cmp::min(d_edge, cmp::min(
          cmp::min(i, h - 1 - i),
          cmp::min(j, w - 1 - j)
        ));
      }
    }
  }

  puts!("{}\n", (d_edge as i32 + k - 1) / k + 1);
}

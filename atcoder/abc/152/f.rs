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

fn dfs(adj: &Vec<Vec<(usize, usize)>>, a: usize, b: usize, prev: usize) -> i64 {
  for &(v, i) in &adj[a] {
    if v == prev {
      continue;
    }
    if v == b {
      return 1 << i as i64;
    }
    let ret = dfs(adj, v, b, a);
    if ret != 0 {
      return ret | (1 << i as i64);
    }
  }
  return 0;
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    es: [(usize1, usize1); n-1],
    m: usize,
    cs: [(usize1, usize1); m],
  }
  let mut adj = vec![vec![]; n];
  for (i, &(a, b)) in es.iter().enumerate() {
    adj[a].push((b, i));
    adj[b].push((a, i));
  }
  let mut path = vec![0i64; m];
  for (i, &(a, b)) in cs.iter().enumerate() {
    path[i] = dfs(&adj, a, b, n);
  }
  let mut ans = 0i64;
  for pat in 0 .. 1 << m {
    let mut white = 0;
    let mut count = 0;
    for (i, &p) in path.iter().enumerate() {
      if (pat >> i) & 1 == 0 {
        continue;
      }
      count += 1;
      white |= p;
    }
    let mut popl = 0;
    for i in 0 .. n {
      if (white >> i) & 1 != 0 {
        popl += 1;
      }
    }
    if count % 2 == 1 {
      ans -= 1 << (n - 1 - popl);
    } else {
      ans += 1 << (n - 1 - popl);
    }
  }
  assert!(ans >= 0);

  puts!("{}\n", ans);
}

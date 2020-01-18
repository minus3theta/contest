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

fn dfs(v: usize, d: usize, depth: &mut Vec<usize>, child: &Vec<Vec<usize>>) {
  depth[v] = d;
  let it = child[v].iter().cloned();
  for c in it {
    dfs(c, d+1, depth, child);
  }
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    p: [i32; n],
    q: usize,
    query: [(usize1, usize1); q],
  }
  let mut root = 0;
  for (i, &x) in p.iter().enumerate() {
    if x == -1 {
      root = i;
      break;
    }
  }
  let root = root;
  let par: Vec<_> = p.into_iter().map(|x| if x == -1 { None } else { Some(x as usize - 1) }).collect();
  let mut log = 0;
  while 1 << log < n {
    log += 1;
  }
  let mut db = vec![vec![None; n]; log + 1];
  let mut child = vec![vec![]; n];
  for v in 0 .. n {
    db[0][v] = par[v];
    if let Some(c) = par[v] {
      child[c].push(v);
    }
  }
  for k in 0 .. log {
    for v in 0 .. n {
      db[k+1][v] = match db[k][v] {
        None => None,
        Some(x) => db[k][x],
      }
    }
  }
  let mut depth = vec![0; n];
  dfs(root, 0, &mut depth, &child);
  for &(mut a, b) in &query {
    if depth[a] <= depth[b] {
      puts!("No\n");
      continue;
    }
    for k in 0 .. log {
      if ((depth[a] - depth[b]) >> k & 1) != 0 {
        match db[k][a] {
          None => {
            puts!("No\n");
            break;
          },
          Some(v) => {
            a = v;
          }
        }
      }
    }
    if a == b {
      puts!("Yes\n");
    } else {
      puts!("No\n");
    }
  }
}

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

fn dfs(v: usize, adj: &Vec<Vec<usize>>, color: &mut Vec<i8>, current: i8) {
  if color[v] != 0 {
    return;
  }
  color[v] = current;
  for &u in &adj[v] {
    dfs(u, adj, color, 3 - current);
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
    es: [(usize1, usize1); n-1],
  }
  let mut adj = vec![vec![]; n];
  for &(a, b) in &es {
    adj[a].push(b);
    adj[b].push(a);
  }
  let mut color = vec![0; n];
  dfs(0, &adj, &mut color, 1);
  let mut c1 = color.iter().filter(|&&c| c == 1).count();
  let mut c2 = n - c1;
  if c2 > c1 {
    for i in 0 .. n {
      color[i] = 3 - color[i];
    }
    std::mem::swap(&mut c1, &mut c2);
  }
  // dbg!()
  // let n1 = (n + 2) / 3;
  let n2 = (n + 1) / 3;
  // let n3 = n / 3;
  let mut p = vec![0; n];
  let mut m1 = 1;
  let mut m2 = 2;
  let mut m3 = 3;
  if n2 <= c2 {
    for i in 0 .. n {
      if color[i] == 1 {
        if m1 <= n {
          p[i] = m1;
          m1 += 3;
        } else {
          p[i] = m3;
          m3 += 3;
        }
      } else {
        if m2 <= n {
          p[i] = m2;
          m2 += 3;
        } else {
          p[i] = m3;
          m3 += 3;
        }
      }
    }
  } else {
    for i in 0 .. n {
      if color[i] == 1 {
        if m1 <= n {
          p[i] = m1;
          m1 += 3;
        } else if m2 <= n {
          p[i] = m2;
          m2 += 3;
        } else {
          p[i] = m3;
          m3 += 3;
        }
      } else {
        p[i] = m3;
        m3 += 3;
      }
    }
  }
  for &x in &p {
    puts!("{} ", x);
  }
  puts!("\n");
}

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

fn fmin(x: f64, y: f64) -> f64 {
  if x < y {
    x
  } else {
    y
  }
}

#[derive(PartialEq, PartialOrd)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
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
    m: usize,
    es: [(usize1, usize1); m],
  }
  let mut exp = vec![vec![0.0; n]; 2];
  let mut adj = vec![vec![]; n];
  for &(s, t) in &es {
    adj[s].push(t);
  }
  for i in (0 .. n - 1).rev() {
    let l = adj[i].len();
    exp[0][i] = 1.0;
    exp[1][i] = 1.0;
    for &j in &adj[i] {
      exp[0][i] += exp[0][j] / l as f64;
      exp[1][i] += exp[1][j] / l as f64;
    }
    if l > 1 {
      let mut next: Vec<_> = adj[i].iter().map(|&x| Total(exp[0][x])).collect();
      next.sort();
      let e1 = 1.0 + next.iter().take(l - 1).map(|x| x.0).sum::<f64>() / (l - 1) as f64;
      exp[1][i] = fmin(exp[1][i], e1);
    }
  }
  dbg!(&exp);

  let ans = fmin(exp[0][0], exp[1][0]);
  puts!("{:.15}\n", ans);
}

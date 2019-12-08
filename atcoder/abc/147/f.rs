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
    n: i64,
    x: i64,
    d: i64,
  }
  if d == 0 {
    if x == 0 {
      puts!("1\n");
    } else {
      puts!("{}\n", n + 1)
    }
    return;
  }
  use std::collections::BTreeMap;
  let mut range = BTreeMap::new();
  for i in 0 .. n + 1 {
    let left = i * (i - 1) / 2;
    let right = i * (2 * n - i - 1) / 2;
    range.entry((i * x) % d).or_insert(vec![]).push((i * x + left * d, i * x + right * d));
  }
  let mut ans = 0i64;
  for v in range.values() {
    let mut rs = vec![];
    for &(l, r) in v {
      if l < r {
        rs.push((l, false));
        rs.push((r, true));
      } else {
        rs.push((l, true));
        rs.push((r, false));
      }
    }
    rs.sort();
    let mut open = 0;
    let mut last = 0;
    for &(p, r) in &rs {
      if r {
        open -= 1;
        if open == 0 {
          ans += (p - last).abs() / d.abs() + 1;
        }
      } else {
        if open == 0 {
          last = p;
        }
        open += 1;
      }
    }
  }

  puts!("{}\n", ans);
}

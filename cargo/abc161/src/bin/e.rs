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

fn fill(n: usize, k: usize, c: usize, s: &Vec<char>) -> usize {
  let mut last = None;
  let mut count = 0;
  for (i, &d) in s.iter().enumerate() {
    if d == 'x' {
      continue;
    }
    if let Some(last) = last {
      if i <= last + c {
        continue;
      }
    }
    last = Some(i);
    count += 1;
  }
  count
}

fn solve(n: usize, k: usize, c: usize, s: &Vec<char>) -> Vec<bool> {
  let mut work = vec![false; n];
  let mut last = None;
  let mut count = 0;
  for (i, &d) in s.iter().enumerate() {
    if count >= k {
      break;
    }
    if d == 'x' {
      continue;
    }
    if let Some(last) = last {
      if i <= last + c {
        continue;
      }
    }
    last = Some(i);
    work[i] = true;
    count += 1;
  }
  work
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    k: usize,
    c: usize,
    s: chars,
  }
  if fill(n, k, c, &s) > k {
    return;
  }
  let forward = solve(n, k, c, &s);
  let mut s = s;
  s.reverse();
  let mut backward = solve(n, k, c, &s);
  backward.reverse();
  for i in 0 .. n {
    if forward[i] && backward[i] {
      puts!("{}\n", i+1);
    }
  }
}

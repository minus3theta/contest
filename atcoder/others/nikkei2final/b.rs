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

fn solve(s: &Vec<char>, s6: usize, s2: usize, s4: usize) -> i64 {
  let n = s.len();
  let l = n - s6;
  if l == 0 {
    return 0;
  }
  let s3 = s2 + l;
  if s3 >= s4 {
    return 0;
  }
  let l3 = s4 - s3;
  if s4 + l3 >= s6 {
    return 0;
  }
  for i in 0 .. l {
    if s[s2 + i] != s[s6 + i] {
      return 0;
    }
  }
  for i in 0 .. l3 {
    if s[s3 + i] != s[s4 + i] {
      return 0;
    }
  }
  1
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    s: chars,
  }
  let mut ans = 0;
  let n = s.len();
  for s6 in 0 .. n {
    for s2 in 1 .. s6 {
      for s4 in s2 .. s6 {
        ans += solve(&s, s6, s2, s4);
      }
    }
  }

  puts!("{}\n", ans);
}

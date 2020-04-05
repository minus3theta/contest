#[allow(unused_imports)]
use std::cmp;
use std::io::Write;

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
#[allow(unused_macros)]
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

fn solve(se: Vec<(i32, i32)>) -> String {
  let mut ise: Vec<_> = se.into_iter().enumerate().collect();
  ise.sort_by_key(|x| (x.1).0);
  let mut assign = vec![false; ise.len()];
  let mut a = 0;
  let mut b = 0;
  for (i, (s, e)) in ise.into_iter() {
    if a <= s {
      a = e;
      assign[i] = true;
    } else if b <= s {
      b = e;
    } else {
      return "IMPOSSIBLE".to_owned();
    }
  }
  assign.iter().map(|&c| if c {'C'} else {'J'}).collect()
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  let s = {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s
  };
  let mut iter = s.split_whitespace();
  input_inner! {
    iter,
    t: usize,
  }
  for x in 0 .. t {
    input_inner! {
      iter,
      n: usize,
      se: [(i32, i32); n],
    }
    let ans = solve(se);
    puts!("Case #{}: {}\n", x+1, ans);
  }
}

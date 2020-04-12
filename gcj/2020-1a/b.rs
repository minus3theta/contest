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

fn solve(n: usize) -> Vec<(usize, usize)> {
  if n <= 31 {
    return (1 .. n+1).map(|x| (x, 1)).collect();
  }
  let mut m = n - 31;
  let mut row = vec![false; 31];
  for i in (0 .. 31).rev() {
    if m >= (1 << i) - 1 {
      m -= (1 << i) - 1;
      row[i] = true;
    }
  }
  let mut ret = vec![(1, 1)];
  let mut left = true;
  for i in 1 .. 31 {
    if row[i] {
      if left {
        for j in 0 .. i+1 {
          ret.push((i+1, j+1));
        }
      } else {
        for j in (0 .. i+1).rev() {
          ret.push((i+1, j+1));
        }
      }
      left = ! left;
    } else {
      if left {
        ret.push((i+1, 1));
      } else {
        ret.push((i+1, i+1));
      }
    }
  }
  let mut i = 32;
  while m > 0 {
    ret.push((i, if left {1} else {i}));
    i += 1;
    m -= 1;
  }

  ret
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    t: usize,
    ns: [usize; t],
  }
  for (i, &n) in ns.iter().enumerate() {
    puts!("Case #{}:\n", i+1);
    for (r, c) in solve(n) {
      puts!("{} {}\n", r, c);
    }
  }
}

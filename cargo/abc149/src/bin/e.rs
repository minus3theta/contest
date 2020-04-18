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
    n: usize,
    m: usize,
    aa: [usize; n],
  }
  let max_a = *aa.iter().max().unwrap();
  let mut sum = vec![0; max_a * 2 + 2];
  for &a in &aa {
    sum[a] += 1;
  }
  for i in 0 .. sum.len() - 1 {
    sum[i+1] += sum[i];
  }
  let mut l = 0;
  let mut r = max_a * 2 + 1;
  while l + 1 != r {
    let x = (l + r) / 2;
    let mut count = 0;
    for &a in &aa {
      count += n - sum[x.saturating_sub(a+1)];
    }
    if count >= m {
      l = x;
    } else {
      r = x;
    }
  }
  // dbg!(l);
  let lb = l;
  let mut count = 0;
  let mut ans = 0;
  for &a in &aa {
    let c = n - sum[lb.saturating_sub(a+1)];
    ans += a * c * 2;
    count += c;
  }
  ans -= lb * (count - m);

  puts!("{}\n", ans);
}

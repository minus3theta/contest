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
    a: [usize1; n],
  }
  let mut val_to_popl = vec![0; n];
  for x in a.into_iter() {
    val_to_popl[x] += 1;
  }
  let mut popl_to_count = vec![0; n + 1];
  for p in val_to_popl.into_iter() {
    if p > 0 {
      popl_to_count[p] += 1;
    }
  }
  let mut sum = vec![0; n + 2];
  for (i, &c) in popl_to_count.iter().enumerate() {
    sum[i+1] = sum[i] + c;
  }
  let mut sum_prod = vec![0; n + 2];
  for (i, &c) in popl_to_count.iter().enumerate() {
    sum_prod[i+1] = sum_prod[i] + i * c;
  }
  let mut count_to_len = vec![0; n + 1];
  for x in 1 .. n + 1 {
    count_to_len[x] = (sum_prod[x+1] + x * (sum[n+1] - sum[x+1])) / x;
  }
  count_to_len[0] = n;
  let mut count = n;
  for k in 1 .. n + 1 {
    while count_to_len[count] < k {
      count -= 1;
    }
    puts!("{}\n", count);
  }
}

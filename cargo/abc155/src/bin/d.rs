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

fn count_less(p: i64, i: usize, v: &Vec<i64>, th: i64) -> usize {
  let mut l = i;
  let mut r = v.len() + 1;
  while l + 1 != r {
    let m = ((l + r) / 2) as usize;
    if p * v[m-1] >= th {
      r = m;
    } else {
      l = m;
    }
  }
  l - i
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
    a: [i64; n],
  }
  let mut pos = vec![];
  let mut neg = vec![];
  let mut zero = 0;
  for x in a.into_iter() {
    if x > 0 {
      pos.push(x);
    } else if x < 0 {
      neg.push(x);
    } else {
      zero += 1;
    }
  }
  pos.sort();
  neg.sort();
  let neg_prod = pos.len() * neg.len();
  let zero_prod = zero * (n - zero) + zero * (zero.saturating_sub(1)) / 2;
  let ans = if k <= neg_prod {
    pos.reverse();
    let mut l = -(1 << 62);
    let mut r = 0;
    while l + 1 != r {
      let x = (l + r) / 2;
      let mut less = 0;
      for &p in &neg {
        less += count_less(p, 0, &pos, x)
      }
      if less < k {
        l = x;
      } else {
        r = x;
      }
    }
    l
  } else if k <= neg_prod + zero_prod {
    0
  } else {
    neg.reverse();
    let k = k - neg_prod - zero_prod;
    let mut l = 0;
    let mut r = 1 << 62;
    while l + 1 != r {
      let x = (l + r) / 2;
      let mut less = 0;
      for (i, &p) in neg.iter().enumerate() {
        less += count_less(p, i+1, &neg, x);
      }
      for (i, &p) in pos.iter().enumerate() {
        less += count_less(p, i+1, &pos, x);
      }
      if less < k {
        l = x;
      } else {
        r = x;
      }
    }
    l
  };

  puts!("{}\n", ans);
}

#[test]
fn test_count1() {
  let v = vec![1, 2];
  assert_eq!(count_less(1, 0, &v, 1), 0)
}
#[test]
fn test_count2() {
  let v = vec![1, 2];
  assert_eq!(count_less(1, 0, &v, 2), 1)
}
#[test]
fn test_count3() {
  let v = vec![1, 2];
  assert_eq!(count_less(1, 0, &v, 3), 2)
}

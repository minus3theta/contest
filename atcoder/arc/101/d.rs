#[allow(unused_imports)]
use std::cmp;

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
    let mut iter = $s.split_whitespace();
    input_inner!{iter, $($r)*}
  };
  ($($r:tt)*) => {
    #[allow(unused_mut)]
    let mut s = {
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

  ($iter:expr, $t:ty) => {
    $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

// 1-indexed
// example: Bit::new(n, |x, y| *x += *y, 0)
struct Bit<T, F> {
    n: usize,
    dat: Vec<T>,
    op: F,
    unit: T,
}

impl<T: Clone, F: Fn(&mut T, &T)> Bit<T, F> {
    #[allow(dead_code)]
    fn new(n: usize, op: F, unit: T) -> Self {
        Bit {
            n: n,
            dat: vec![unit.clone(); n + 1],
            op: op,
            unit: unit,
        }
    }
    #[allow(dead_code)]
    fn from_vec(mut v: Vec<T>, op: F, unit: T) -> Self {
        let n = v.len();
        let mut dat = vec![unit.clone()];
        dat.append(&mut v);
        for i in 1..n {
            let j = i as i32;
            let b = (j & -j) as usize;
            let x = dat[i].clone();
            op(&mut dat[i + b], &x);
        }
        Bit {
            n: n,
            dat: dat,
            op: op,
            unit: unit,
        }
    }
    fn operate(&mut self, k: usize, a: &T) {
        let mut k = k;
        while k <= self.n {
            (self.op)(&mut self.dat[k], &a);
            let l = k as i32;
            k += (l & -l) as usize;
        }
    }
    fn accum(&self, k: usize) -> T {
        let mut k = k;
        let mut sum = self.unit.clone();
        while k > 0 {
            (self.op)(&mut sum, &self.dat[k]);
            let l = k as i32;
            k -= (l & -l) as usize;
        }
        sum
    }
}

fn solve(aa: &Vec<i64>, m: i64) -> bool {
  let n = aa.len();
  let bb: Vec<i64> = aa.iter().map(|&a| if a >= m { 1 } else { -1 }).collect();
  let mut cc = vec![0i64; n + 1];
  for (i, &b) in bb.iter().enumerate() {
    cc[i+1] = cc[i] + b;
  }
  let mut count = 0i64;
  let mut bit = Bit::new(n + 1, |x, y| *x += *y, 0i64);
  let mut dd: Vec<_> = cc.iter().enumerate().map(|(i, &c)| (c, i)).collect();
  dd.sort();
  for (_, i) in dd {
    count += bit.accum(i);
    bit.operate(i + 1, &1);
  }
  let n = n as i64;
  count >= (n * (n + 1) / 2 + 1) / 2
}

fn main() {
  input! {
    n: usize,
    aa: [i64; n]
  }
  let mut l = 0i64;
  let mut r = 1_000_000_001i64;
  while l + 1 < r {
    let m = (l + r) / 2;
    if solve(&aa, m) {
      l = m;
    } else {
      r = m;
    }
  }
  println!("{}", l);
}

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

const INF: i64 = 1 << 60;

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    a: [i32; n],
    b: [i32; n],
  }
  let mut ans = INF;
  for pat in 0 .. (1 << n) {
    let mut flip = [0, 0];
    for i in 0 .. n {
      if (pat >> i) & 1 != 0 {
        flip[i % 2] += 1;
      }
    }
    if flip[0] != flip[1] {
      continue;
    }
    let mut nums = vec![vec![]; 2];
    for i in 0 .. n {
      let flipped = (pat >> i) & 1 != 0;
      let v = if flipped {
        b[i]
      } else {
        a[i]
      };
      if flipped {
        nums[(i + 1) % 2].push((v, i));
      } else {
        nums[i % 2].push((v, i));
      }
    }
    nums[0].sort();
    nums[1].sort();
    let mut sorted = true;
    for i in 0 .. nums[1].len() {
      if nums[0][i].0 > nums[1][i].0 {
        sorted = false;
        break;
      }
    }
    for i in 1 .. nums[0].len() {
      if nums[1][i-1].0 > nums[0][i].0 {
        sorted = false;
        break;
      }
    }
    if ! sorted {
      continue;
    }
    let mut bit = Bit::new(n, |x, y| *x += *y, 0);
    let mut swap = 0;
    for i in (0 .. n).rev() {
      let index = nums[i % 2][i / 2].1 + 1;
      swap += bit.accum(index);
      bit.operate(index, &1);
    }
    ans = cmp::min(ans, swap);
  }

  puts!("{}\n", if ans == INF { -1 } else { ans });
}

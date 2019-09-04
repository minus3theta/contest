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

const MOD: i64 = 998244353;

#[derive(Copy, Clone, Hash, Debug)]
struct ModInt {
  x: i64
}
impl ModInt {
  fn new(x: i64) -> Self {
    ModInt {
      x: x % MOD
    }
  }
  fn pow(self, e: i64) -> Self {
    if e == 0 {
      Self::new(1)
    } else if e % 2 == 1 {
      self.pow(e - 1) * self
    } else {
      let y = self.pow(e / 2);
      y * y
    }
  }
  #[allow(dead_code)]
  fn inv(self) -> Self {
    self.pow(MOD - 2)
  }
}
impl<T: Into<ModInt>> std::ops::Add<T> for ModInt {
  type Output = Self;
  fn add(self, other: T) -> Self {
    let other = other.into();
    let sum = self.x + other.x;
    ModInt {
      x: if sum >= MOD { sum - MOD } else { sum }
    }
  }
}
impl<T: Into<ModInt>> std::ops::Sub<T> for ModInt {
  type Output = Self;
  fn sub(self, other: T) -> Self {
    let other = other.into();
    let sum = self.x - other.x;
    ModInt {
      x: if sum < 0 { sum + MOD } else { sum }
    }
  }
}
impl<T: Into<ModInt>> std::ops::Mul<T> for ModInt {
  type Output = Self;
  fn mul(self, other: T) -> Self {
    let other = other.into();
    ModInt {
      x: self.x * other.x % MOD
    }
  }
}
impl<T: Into<ModInt>> std::ops::Div<T> for ModInt {
  type Output = Self;
  fn div(self, other: T) -> Self {
    let other = other.into();
    self * other.inv()
  }
}
impl From<i64> for ModInt {
  fn from(x: i64) -> Self {
    ModInt::new(x)
  }
}
impl std::fmt::Display for ModInt {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    self.x.fmt(f)
  }
}
impl<T: Into<ModInt>> std::ops::AddAssign<T> for ModInt {
  fn add_assign(&mut self, other: T) {
    *self = *self + other;
  }
}
impl<T: Into<ModInt>> std::ops::SubAssign<T> for ModInt {
  fn sub_assign(&mut self, other: T) {
    *self = *self - other;
  }
}
impl<T: Into<ModInt>> std::ops::MulAssign<T> for ModInt {
  fn mul_assign(&mut self, other: T) {
    *self = *self * other;
  }
}
impl<T: Into<ModInt>> std::ops::DivAssign<T> for ModInt {
  fn div_assign(&mut self, other: T) {
    *self = *self / other;
  }
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    ps: [(i64, i64); n],
  }
  let mut xs: Vec<_> = ps.iter().map(|&(x, _)| x).collect();
  xs.sort();
  use std::collections::BTreeMap;
  let x_compress: BTreeMap<_, _> = xs.iter().enumerate().map(|(i, &x)| (x, i)).collect();
  let mut ps = ps;
  ps.sort_by_key(|&(_, y)| y);
  let left = {
    let mut left = vec![(0, 0); n];
    let mut bit = Bit::new(n, |x, y| *x += *y, 0);
    for (i, &(x, _)) in ps.iter().enumerate() {
      let xc = *x_compress.get(&x).unwrap() + 1;
      let bottom = bit.accum(xc);
      left[i] = (i - bottom, bottom);
      bit.operate(xc, &1);
    }
    left
  };
  let right = {
    let mut right = vec![(0, 0); n];
    let mut bit = Bit::new(n, |x, y| *x += *y, 0);
    for (i, &(x, _)) in ps.iter().enumerate().rev() {
      let xc = *x_compress.get(&x).unwrap() + 1;
      let bottom = bit.accum(xc);
      right[i] = (n - 1 - i - bottom, bottom);
      bit.operate(xc, &1);
    }
    right
  };
  let mut ans = ModInt::new(0);
  for (&(tl, bl), &(tr, br)) in left.iter().zip(right.iter()) {
    let ptl = ModInt::new(2).pow(tl as i64) - 1;
    let pbl = ModInt::new(2).pow(bl as i64) - 1;
    let ptr = ModInt::new(2).pow(tr as i64) - 1;
    let pbr = ModInt::new(2).pow(br as i64) - 1;
    // 0
    ans += 1;
    // 1
    ans += ptl + pbl + ptr + pbr;
    // 2 - straight
    ans += ptl * pbl + pbl * pbr + pbr * ptr + ptr * ptl;
    // 2 - cross
    ans += (ptl * pbr + pbl * ptr) * 2;
    // 3
    ans += ptl * pbl * ptr       * 2;
    ans += ptl * pbl       * pbr * 2;
    ans += ptl       * ptr * pbr * 2;
    ans +=       pbl * ptr * pbr * 2;
    // 4
    ans += ptl * pbl * ptr * pbr * 2;
  }

  puts!("{}\n", ans);
}

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

const MOD: i64 = 1_000_000_007;

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

fn get_pows(a: ModInt, n: usize) -> Vec<ModInt> {
  let mut ret = vec![ModInt::new(1); n];
  for i in 1 .. n {
    ret[i] = ret[i-1] * a;
  }
  ret
}

fn main() {
  input! {
    n: usize,
    a: i64,
    b: i64,
    _c: i64
  }
  let a = ModInt::new(a) / 100;
  let b = ModInt::new(b) / 100;
  let end_m = 2 * n;
  let mut fact = vec![ModInt::new(1); end_m];
  for i in 1 .. end_m {
    fact[i] = fact[i-1] * i as i64;
  }
  let mut inv_fact = vec![ModInt::new(0); end_m];
  inv_fact[end_m - 1] = fact[end_m - 1].inv();
  for i in (1 .. end_m).rev() {
    inv_fact[i-1] = inv_fact[i] * i as i64;
  }
  let powa = get_pows(a / (a + b), n + 1);
  let powb = get_pows(b / (a + b), n + 1);

  let mut ans = ModInt::new(0);
  for m in n .. end_m {
    ans += fact[m-1] * inv_fact[n-1] * inv_fact[m-n] *
      (powa[n] * powb[m-n] + powa[m-n] * powb[n]) * m as i64 / (a + b);
  }

  println!("{}", ans.x);
}

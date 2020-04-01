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

const MOD: i64 = 1_000_000_007;

#[derive(Copy, Clone, Hash, Debug)]
struct ModInt {
  x: i64
}
impl ModInt {
  fn new(x: i64) -> Self {
    let x = x % MOD;
    ModInt {
      x: if x < 0 {
        x + MOD
      } else {
        x
      }
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
  #[allow(dead_code)]
  fn fact(n: usize) -> Vec<Self> {
    let mut ret = vec![0.into(); n+1];
    ret[0] = 1.into();
    for i in 1 .. n + 1 {
      ret[i] = ret[i-1] * i as i64;
    }
    ret
  }
  #[allow(dead_code)]
  fn inv_fact(fact: &Vec<Self>) -> Vec<Self> {
    let n = fact.len() - 1;
    let mut ret = vec![0.into(); n+1];
    ret[n] = fact[n].inv();
    for i in (0 .. n).rev() {
      ret[i] = ret[i+1] * (i + 1) as i64;
    }
    ret
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

fn bottom_up(v: usize, prev: usize, adj: &Vec<Vec<usize>>, fact: &Vec<ModInt>, inv_fact: &Vec<ModInt>, sub: &mut Vec<(usize, ModInt)>) -> (usize, ModInt) {
  let mut prod = ModInt::new(1);
  let mut size = 0;
  for &c in &adj[v] {
    if c == prev {
      continue;
    }
    let (s, p) = bottom_up(c, v, adj, fact, inv_fact, sub);
    prod *= p * inv_fact[s];
    size += s;
  }
  prod *= fact[size];
  sub[v] = (size+1, prod);
  sub[v]
}

fn top_down(v: usize, prev: usize, parent: (usize, ModInt), adj: &Vec<Vec<usize>>, fact: &Vec<ModInt>, inv_fact: &Vec<ModInt>, sub: &Vec<(usize, ModInt)>, ans: &mut Vec<ModInt>) {
  // dbg!((v, parent));
  let (size, mut prod) = parent;
  prod *= inv_fact[size];
  for &c in &adj[v] {
    if c == prev {
      continue;
    }
    let (s, p) = sub[c];
    prod *= p * inv_fact[s];
  }
  ans[v] = prod * fact[adj.len()-1];
  for &c in &adj[v] {
    if c == prev {
      continue;
    }
    let (s, p) = sub[c];
    top_down(c, v, (adj.len()-s, prod / p * fact[s] * fact[adj.len()-s-1]), adj, fact, inv_fact, sub, ans);
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
    es: [(usize1, usize1); n-1],
  }
  let mut adj = vec![vec![]; n];
  for &(a, b) in &es {
    adj[a].push(b);
    adj[b].push(a);
  }
  let fact = ModInt::fact(n);
  let inv_fact = ModInt::inv_fact(&fact);
  let mut sub = vec![(0, 0.into()); n];
  bottom_up(0, n, &adj, &fact, &inv_fact, &mut sub);
  let mut ans = vec![0.into(); n];
  top_down(0, n, (0, 1.into()), &adj, &fact, &inv_fact, &sub, &mut ans);

  for &x in &ans {
    puts!("{}\n", x);
  }
}

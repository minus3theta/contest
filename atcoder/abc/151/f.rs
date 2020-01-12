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

#[allow(dead_code)]
const EPS: f64 = 1e-8;

#[derive(Copy, Clone, Debug)]
struct Point {
  x: f64,
  y: f64,
}
impl Point {
  #[allow(dead_code)]
  fn new(x: f64, y: f64) -> Self {
    Point {
      x: x,
      y: y,
    }
  }
  #[allow(dead_code)]
  fn division(self, other: Self, ratio: f64) -> Self {
    self + (other - self) * ratio
  }
  #[allow(dead_code)]
  fn dist(self, other: Self) -> f64 {
    (other - self).abs()
  }
  #[allow(dead_code)]
  fn line_toward(self, vec: Vect) -> Line {
    Line {
      a: vec.y,
      b: -vec.x,
      c: self.x * vec.y - self.y * vec.x,
    }
  }
  #[allow(dead_code)]
  fn line_between(self, other: Point) -> Line {
    let vec = (other - self).normalized();
    self.line_toward(vec)
  }
  #[allow(dead_code)]
  fn bisector(self, other: Point) -> Line {
    let midpoint = self.division(other, 0.5);
    let vec = (other - self).normal_vect();
    midpoint.line_toward(vec)
  }
}
impl<T: Into<Vect>> std::ops::Add<T> for Point {
  type Output = Point;
  fn add(self, other: T) -> Point {
    let other = other.into();
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}
impl std::ops::Sub<Point> for Point {
  type Output = Vect;
  fn sub(self, other: Point) -> Vect {
    Vect {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}
impl<T: Into<Vect>> std::ops::Sub<T> for Point {
  type Output = Point;
  fn sub(self, other: T) -> Point {
    let other = other.into();
    Point {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}
impl From<(f64, f64)> for Point {
  fn from(p: (f64, f64)) -> Self {
    Point::new(p.0, p.1)
  }
}

#[derive(Copy, Clone, Debug)]
struct Vect {
  x: f64,
  y: f64,
}
impl Vect {
  #[allow(dead_code)]
  fn new(x: f64, y: f64) -> Vect {
    Vect {
      x: x,
      y: y,
    }
  }
  #[allow(dead_code)]
  fn from_angle(theta: f64) -> Vect {
    Vect {
      x: theta.cos(),
      y: theta.sin(),
    }
  }
  #[allow(dead_code)]
  fn norm(self) -> f64 {
    self.x.powi(2) + self.y.powi(2)
  }
  #[allow(dead_code)]
  fn abs(self) -> f64 {
    self.norm().sqrt()
  }
  #[allow(dead_code)]
  fn normal_vect(self) -> Vect {
    Vect {
      x: -self.y,
      y: self.x,
    }
  }
  #[allow(dead_code)]
  fn normalized(self) -> Vect {
    self / self.abs()
  }
}
impl<T: Into<Vect>> std::ops::Add<T> for Vect {
  type Output = Vect;
  fn add(self, other: T) -> Vect {
    let other = other.into();
    Vect {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}
impl<T: Into<Vect>> std::ops::Sub<T> for Vect {
  type Output = Vect;
  fn sub(self, other: T) -> Vect {
    let other = other.into();
    Vect {
      x: self.x - other.x,
      y: self.y - other.y,
    }
  }
}
impl std::ops::Mul<f64> for Vect {
  type Output = Vect;
  fn mul(self, other: f64) -> Vect {
    Vect {
      x: self.x * other,
      y: self.y * other,
    }
  }
}
impl std::ops::Div<f64> for Vect {
  type Output = Vect;
  fn div(self, other: f64) -> Vect {
    Vect {
      x: self.x / other,
      y: self.y / other,
    }
  }
}
impl<T: Into<Vect>> std::ops::AddAssign<T> for Vect {
  fn add_assign(&mut self, other: T) {
    *self = *self + other;
  }
}
impl<T: Into<Vect>> std::ops::SubAssign<T> for Vect {
  fn sub_assign(&mut self, other: T) {
    *self = *self - other;
  }
}
impl std::ops::MulAssign<f64> for Vect {
  fn mul_assign(&mut self, other: f64) {
    *self = *self * other;
  }
}
impl From<(f64, f64)> for Vect {
  fn from(p: (f64, f64)) -> Self {
    Vect::new(p.0, p.1)
  }
}

#[derive(Copy, Clone, Debug)]
struct Line {
  // ax + by = c
  a: f64,
  b: f64,
  c: f64,
}
impl Line {
  #[allow(dead_code)]
  fn new(a: f64, b: f64, c: f64) -> Self {
    Line {
      a: a,
      b: b,
      c: c,
    }
  }
  #[allow(dead_code)]
  fn intersection(self, other: Line) -> Option<Point> {
    let det = self.a * other.b - self.b * other.a;
    if det.abs() < EPS {
      return None;
    }
    Some(Point {
      x: (other.b * self.c - self.b * other.c) / det,
      y: (-other.a * self.c + self.a * other.c) / det,
    })
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
    ps: [(f64, f64); n],
  }
  let ps: Vec<Point> = ps.into_iter().map(|p| p.into()).collect();
  let mut ans = 1e100;
  for i in 0 .. n {
    for j in i+1 .. n {
      let center = ps[i].division(ps[j], 0.5);
      let mut max = 0.0;
      for l in 0 .. n {
        let d = ps[l].dist(center);
        if d > max {
          max = d;
        }
      }
      if max < ans {
        ans = max;
      }
    }
  }
  for i in 0 .. n {
    for j in i+1 .. n {
      let l1 = ps[i].bisector(ps[j]);
      for k in j+1 .. n {
        let l2 = ps[i].bisector(ps[k]);
        if let Some(center) = l1.intersection(l2) {
          let mut max = 0.0;
          for l in 0 .. n {
            let d = ps[l].dist(center);
            if d > max {
              max = d;
            }
          }
          if max < ans {
            ans = max;
          }
        }
      }
    }
  }

  puts!("{:.15}\n", ans);
}

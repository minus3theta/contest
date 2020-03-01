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

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Total<T>(pub T);

impl<T: PartialEq> Eq for Total<T> {}

impl<T: PartialOrd> Ord for Total<T> {
    fn cmp(&self, other: &Total<T>) -> std::cmp::Ordering {
        self.0.partial_cmp(&other.0).unwrap()
    }
}

#[allow(dead_code)]
const EPS: f64 = 1e-6;

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
  fn line_with_normal(self, vec: Vect) -> Line {
    let vec = vec.normalized();
    Line {
      a: vec.x,
      b: vec.y,
      c: -self.x * vec.x - self.y * vec.y,
    }
  }
  #[allow(dead_code)]
  fn line_toward(self, vec: Vect) -> Line {
    self.line_with_normal(vec.normal_vect())
  }
  #[allow(dead_code)]
  fn line_between(self, other: Point) -> Line {
    self.line_toward(other - self)
  }
  #[allow(dead_code)]
  fn bisector(self, other: Point) -> Line {
    let midpoint = self.division(other, 0.5);
    midpoint.line_with_normal(other - self)
  }
  #[allow(dead_code)]
  fn apollonius(self, other: Point, r1: f64, r2: f64) -> Result<Circle, Line> {

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
  // ax + by + c = 0
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
      x: (-other.b * self.c + self.b * other.c) / det,
      y: (other.a * self.c - self.a * other.c) / det,
    })
  }
}
#[derive(Copy, Clone, Debug)]
struct Circle {
  c: Point,
  r: f64,
}
impl Circle {
  #[allow(dead_code)]
  fn new(c: Point, r: f64) -> Self {
    Circle {
      c: c,
      r: r
    }
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
    k: usize,
    meat: [(f64, f64, f64); n],
  }
  let meat: Vec<_> = meat.into_iter().map(|(a, b, c)| (Point::new(a, b), c)).collect();
  let time = |p: Point| {
    let mut times = vec![];
    for &(q, c) in &meat {
      let x = p.dist(q) * c;
      times.push(Total(x));
    }
    times.sort();
    times[k-1]
  };
  let mut ans = Total(1.0 / 0.0);
  for i in 0 .. n {
    ans = cmp::min(ans, time(meat[i].0));
  }
  for i in 0 .. n {
    let (pi, ci) = meat[i];
    for j in 0 .. i {
      let (pj, cj) = meat[j];
      let p = pi.division(pj, cj / (ci + cj));
      ans = cmp::min(ans, time(p));
    }
  }
  let get_line = |(p1, c1): (Point, f64), (p2, c2): (Point, f64)| {
    p1.division(p2, c2 / (c1 + c2)).line_with_normal(p2 - p1)
  };
  for i in 0 .. n {
    for j in 0 .. i {
      let l1 = get_line(meat[i], meat[j]);
      for l in 0 .. j {
        let l2 = get_line(meat[i], meat[l]);
        let q = l1.intersection(l2);
        dbg!((l1, l2, q));
        if let Some(q) = q {
          ans = cmp::min(ans, time(q));
        }
      }
    }
  }

  puts!("{:.15}\n", ans.0);
}

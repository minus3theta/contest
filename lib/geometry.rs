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
  // divide a segment into; ratio : 1 - ratio
  fn division(self, other: Self, ratio: f64) -> Self {
    self + (other - self) * ratio
  }
  #[allow(dead_code)]
  fn dist(self, other: Self) -> f64 {
    (other - self).abs()
  }
  #[allow(dead_code)]
  fn line_with_normal(self, vec: Vect) -> Line {
    Line::new(vec.x, vec.y, -self.x * vec.x - self.y * vec.y)
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
  fn dist_to_line(self, line: Line) -> f64 {
    (line.a * self.x + line.b * self.y + line.c).abs()
  }
  #[allow(dead_code)]
  fn apollonius(self, other: Point, r1: f64, r2: f64) -> Option<Curve> {
    use Curve::*;
    if self.dist(other) < EPS || r1 < EPS || r2 < EPS {
      return None;
    }
    if (r1 - r2).abs() < EPS {
      return Some(CLine(self.bisector(other)));
    }
    let p1 = self.division(other, r1 / (r1 + r2));
    let p2 = self.division(other, r1 / (r1 - r2));
    Some(CCircle(Circle::new(p1.division(p2, 0.5), p1.dist(p2) / 2.0)))
  }
  #[allow(dead_code)]
  fn perpendicular(self, line: Line) -> Line {
    self.line_toward(line.normal_vect())
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
    let scale = (a.powi(2) + b.powi(2)).sqrt();
    Line {
      a: a / scale,
      b: b / scale,
      c: c / scale,
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
  #[allow(dead_code)]
  fn normal_vect(self) -> Vect {
    Vect::new(self.a, self.b)
  }
}

#[derive(Copy, Clone, Debug)]
struct Circle {
  center: Point,
  radius: f64,
}
impl Circle {
  #[allow(dead_code)]
  fn new(center: Point, radius: f64) -> Self {
    Circle {
      center: center,
      radius: radius
    }
  }
  #[allow(dead_code)]
  fn intersection_with_line(self, line: Line) -> Vec<Point> {
    let perpendicular = self.center.perpendicular(line);
    let foot = line.intersection(perpendicular).unwrap();
    let dist = self.center.dist_to_line(line);
    if (dist - self.radius).abs() < EPS {
      return vec![foot];
    }
    if dist > self.radius {
      return vec![];
    }
    let base = (self.radius.powi(2) - dist.powi(2)).sqrt();
    let v = perpendicular.normal_vect() * base;
    vec![foot + v, foot - v]
  }
  #[allow(dead_code)]
  fn intersection_with_circle(self, other: Circle) -> Vec<Point> {
    let dist = self.center.dist(other.center);
    if dist < EPS {
      return vec![];
    }
    fn diff(c: Circle) -> f64 {
      c.radius.powi(2) - c.center.x.powi(2) - c.center.y.powi(2)
    }
    let c = (diff(self) - diff(other)) / 2.0;
    let v = self.center - other.center;
    let line = Line::new(v.x, v.y, c);
    self.intersection_with_line(line)
  }
}

#[derive(Copy, Clone, Debug)]
enum Curve {
  CLine(Line),
  CCircle(Circle),
}
impl Curve {
  #[allow(dead_code)]
  fn intersection(self, other: Curve) -> Vec<Point> {
    use Curve::*;
    match (self, other) {
      (CLine(l1), CLine(l2)) => {
        l1.intersection(l2).into_iter().collect()
      },
      (CLine(l), CCircle(c)) | (CCircle(c), CLine(l)) => {
        c.intersection_with_line(l)
      },
      (CCircle(c1), CCircle(c2)) => {
        c1.intersection_with_circle(c2)
      }
    }
  }
}

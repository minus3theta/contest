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

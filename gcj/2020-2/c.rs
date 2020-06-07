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

fn sub(p: (i64, i64), q: (i64, i64)) -> (i64, i64) {
  (p.0 - q.0, p.1 - q.1)
}

fn gcd(x: i64, y: i64) -> i64 {
  if y == 0 {
    x
  } else {
    gcd(y, x % y)
  }
}


use std::collections::BTreeMap;

fn solve(xys: &Vec<(i64, i64)>) -> i64 {
  let mut ans = 1;
  let n = xys.len();
  for i in 0..n {
    for j in 0..i {
      let (a, b) = {
        let (dx, dy) = sub(xys[j], xys[i]);
        let g = gcd(dx, dy);
        (dy / g, - dx / g)
      };
      let mut pop = BTreeMap::new();
      for k in 0..n {
        let (x, y) = xys[k];
        let grp = a * x + b * y;
        *pop.entry(grp).or_insert(0i64) += 1;
      }
      let mut h = 0;
      let mut rem = 0;
      // dbg!(((a, b), &pop));
      for &p in pop.values() {
        h += p / 2 * 2;
        if p >= 3 && p % 2 == 1 {
          rem += 1;
        }
      }
      h += rem / 2 * 2;
      ans = cmp::max(ans, cmp::min(n as i64, h + 2));
    }
  }
  ans
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    t: usize,
    xy: [[(i64, i64)]; t],
  }
  for (case, xys) in xy.iter().enumerate() {
    puts!("Case #{}: {}\n", case+1, solve(xys));
  }
}

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

fn main() {
  input! {
    n: usize,
    k: usize,
    query: [(usize, usize, chars); n],
  }
  let m = k * 2;
  let mut field = vec![vec![0i64; k * 3]; k * 2];
  for &(x, y, ref c) in &query {
    let y = match *c.first().unwrap() {
      'B' => y,
      _ => y + k,
    };
    let (x, y) = if (x % m) < k { (x % m, y % m) } else { ((x + k) % m, (y + k) % m) };
    field[y][x] += 1;
    field[(y + k) % m][x + k] += 1;
    field[y][x + m] += 1;
  }
  for i in 1 .. 2 * k {
    for j in 0 .. 3 * k {
      field[i][j] += field[i-1][j];
    }
  }
  for i in 0 .. 2 * k {
    for j in 1 .. 3 * k {
      field[i][j] += field[i][j-1];
    }
  }
  let mut ans = 0;
  for i in 0 .. k {
    for j in 0 .. 2 * k {
      ans = cmp::max(ans, field[i + k][j + k] - field[i][j + k] - field[i + k][j] + field[i][j]);
    }
  }

  println!("{}", ans);
}

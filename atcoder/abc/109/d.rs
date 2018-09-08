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
  input!{
    h: usize,
    w: usize,
    fld: [[i32; w]; h]
  }
  let mut fld = fld;
  let mut ops = vec![];
  for i in 0..h {
    if i % 2 == 0 {
      for j in 0..w-1 {
        if fld[i][j] % 2 == 1 {
          fld[i][j+1] += 1;
          ops.push((i, j, i, j+1));
        }
      }
      if i + 1 < h && fld[i][w-1] % 2 == 1 {
        fld[i+1][w-1] += 1;
        ops.push((i, w-1, i+1, w-1));
      }
    } else {
      for j in (1..w).rev() {
        if fld[i][j] % 2 == 1 {
          fld[i][j-1] += 1;
          ops.push((i, j, i, j-1));
        }
      }
      if i + 1 < h && fld[i][0] % 2 == 1 {
        fld[i+1][0] += 1;
        ops.push((i, 0, i+1, 0));
      }
    }
  }
  println!("{}", ops.len());
  for &(a, b, c, d) in &ops {
    println!("{} {} {} {}", a+1, b+1, c+1, d+1);
  }
}

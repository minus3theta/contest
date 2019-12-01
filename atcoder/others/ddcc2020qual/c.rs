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

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    h: usize,
    w: usize,
    _k: usize,
    s: [chars; h],
  }
  let mut id = 1;
  let mut field = vec![vec![0; w]; h];
  for i in 0 .. h {
    let sharps: Vec<_> = s[i].iter().enumerate().filter(|&(_, &c)| c == '#').map(|(i, _)| i).collect();
    if sharps.is_empty() {
      continue;
    }
    let mut j = 0;
    for &s in &sharps {
      while j <= s {
        field[i][j] = id;
        j += 1;
      }
      id += 1;
    }
    while j < w {
      field[i][j] = id - 1;
      j += 1;
    }
  }
  let mut row = field.iter().filter(|&r| r[0] != 0).next().unwrap().clone();
  for i in 0 .. h {
    if field[i][0] == 0 {
      field[i] = row.clone();
    } else {
      row = field[i].clone();
    }
  }
  for row in &field {
    for &x in row {
      puts!("{} ", x);
    }
    puts!("\n");
  }
}

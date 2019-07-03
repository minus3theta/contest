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
    _n: usize,
    s: chars,
    q: usize,
    query: [usize; q],
  }
  for &k in &query {
    let mut ans = 0i64;
    let mut d = 0;
    let mut m = 0;
    let mut dm = 0;
    for (i, &c) in s.iter().enumerate() {
      if let Some(out) = i.checked_sub(k) {
        match s[out] {
          'D' => {
            d -= 1;
            dm -= m;
          },
          'M' => {
            m -= 1;
          }
          _ => (),
        }
      }
      match c {
        'D' => {
          d += 1;
        },
        'M' => {
          m += 1;
          dm += d;
        },
        'C' => {
          ans += dm;
        },
        _ => (),
      }
    }
    puts!("{}\n", ans);
  }
}

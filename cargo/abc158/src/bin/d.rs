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
  let s = {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s).unwrap();
    s
  };
  let mut iter = s.split_whitespace();
  input_inner! {
    iter,
    s: chars,
    q: usize,
  }
  use std::collections::VecDeque;
  let mut ss = VecDeque::new();
  let mut flip = false;
  ss.push_back(s);
  for _ in 0 .. q {
    input_inner! {
      iter,
      t: i32,
    }
    if t == 1 {
      flip = ! flip;
    } else {
      input_inner! {
        iter,
        f: i32,
        c: chars,
      }
      let mut c = c;
      if flip {
        c.reverse();
        if f == 1 {
          ss.push_back(c);
        } else {
          ss.push_front(c);
        }
      } else {
        if f == 1 {
          ss.push_front(c);
        } else {
          ss.push_back(c);
        }
      }
    }
  }
  if flip {
    for s in ss.iter().rev() {
      for c in s.iter().rev() {
        puts!("{}", c);
      }
    }
  } else {
    for s in &ss {
      for c in s {
        puts!("{}", c);
      }
    }
  }

  puts!("\n");
}

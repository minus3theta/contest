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
    n: usize,
    s: chars,
  }
  if n % 2 == 1 {
    let mut l = 1i64;
    let mut r = n as i64;
    let mut i = 0;
    while l <= r {
      if s[i] == 'L' {
        puts!("{}\n", l);
        l += 2;
      } else {
        puts!("{}\n", r);
        r -= 2;
      }
      i += 1;
    }
    l = 2;
    r = n as i64 - 1;
    while i < n {
      if s[i] == 'L' {
        puts!("{}\n", l);
        l += 2;
      } else {
        puts!("{}\n", r);
        r -= 2;
      }
      i += 1;
    }
  } else {
    let mut l = 1i64;
    let mut r = n as i64;
    let mut i = 0;
    use std::collections::VecDeque;
    let mut vacant = Vec::new();
    while l <= r {
      if s[i] == 'L' {
        puts!("{}\n", l);
        vacant.push(l + 1);
        l += 2;
      } else {
        puts!("{}\n", r);
        vacant.push(r - 1);
        r -= 2;
      }
      i += 1;
    }
    vacant.sort();
    let mut vacant: VecDeque<_> = vacant.into_iter().collect();
    while i < n {
      if s[i] == 'L' {
        let l = vacant.pop_front().unwrap();
        puts!("{}\n", l);
      } else {
        let r = vacant.pop_back().unwrap();
        puts!("{}\n", r);
      }
      i += 1;
    }
  }

}

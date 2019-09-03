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

fn pow(x: i32, y: i32, p: i32) -> i32 {
  if y == 0 {
    x
  } else if y % 2 == 1 {
    (pow(x, y - 1, p) * x) % p
  } else {
    let a = pow(x, y / 2, p);
    a * a % p
  }
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    p: i32,
    a: [i32; p as usize],
  }
  let pu = p as usize;
  let mut fact = vec![1; pu];
  for i in 1 .. pu {
    fact[i] = (fact[i-1] * i as i32) % p;
  }
  let mut inv_fact = vec![1; pu];
  inv_fact[pu-1] = pow(fact[pu-1], p-1, p);
  for i in (1 .. pu).rev() {
    inv_fact[i-1] = (inv_fact[i] * i as i32) % p;
  }
  let mut ans = vec![0; pu];
  for (i, &x) in a.iter().enumerate() {
    if x == 1 {
      ans[0] = (ans[0] + 1) % p;
      let mut prod = 1;
      for k in 0 .. pu {
        let coef = fact[pu-1] * inv_fact[k] % p * inv_fact[pu-1-k] % p * prod % p;
        if k % 2 == 0 {
          ans[pu-1-k] = (ans[pu-1-k] + p - coef) % p;
        } else {
          ans[pu-1-k] = (ans[pu-1-k] + coef) % p;
        }
        prod = (prod * i as i32) % p;
      }
    }
  }
  for &x in &ans {
    puts!("{} ", x);
  }
  puts!("\n");
}

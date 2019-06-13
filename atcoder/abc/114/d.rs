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

fn eratosthenes(n: i64) -> Vec<i64> {
  let mut primes = vec![];
  for i in 2 .. n + 1 {
    let mut is_prime = true;
    for &p in &primes {
      if p * p > i {
        break;
      }
      if i % p == 0 {
        is_prime = false;
        break;
      }
    }
    if is_prime {
      primes.push(i);
    }
  }
  primes
}

fn count_prime(mut n: i64, p: i64) -> i64 {
  let mut exp = 0;
  while n % p == 0 {
    exp += 1;
    n /= p;
  }
  exp
}

fn count_prime_all(n: i64, p: i64) -> i64 {
  (1..n+1).map(|x| count_prime(x, p)).sum()
}

fn main() {
  input! {
    n: i64,
  }
  let primes = eratosthenes(n);
  let prime_count = primes.iter().map(|p| count_prime_all(n, *p)).collect::<Vec<_>>();
  let c74 = prime_count.iter().filter(|&c| *c >= 74).count();
  let c24 = prime_count.iter().filter(|&c| *c >= 24).count();
  let c14 = prime_count.iter().filter(|&c| *c >= 14).count();
  let c4 = prime_count.iter().filter(|&c| *c >= 4).count();
  let c2 = prime_count.iter().filter(|&c| *c >= 2).count();
  let ans = c74 +
            c24 * (c2 - 1) +
            c14 * (c4 - 1) +
            c4 * (c4 - 1) * (c2 - 2) / 2;

  println!("{}", ans);
}

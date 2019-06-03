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
    x: usize,
  }
  if x == 1 || x == 2 * n - 1 {
    println!("No");
    return;
  }
  println!("Yes");
  if n == 2 {
    println!("1\n2\n3");
    return;
  }
  let mut ans = vec![0; 2 * n - 1];
  if x == 2 {
    ans[n-2] = x + 1;
    ans[n-1] = x;
    ans[n] = x - 1;
    ans[n+1] = x + 2;
    let mut v = 1;
    for i in 0 .. n - 2 {
      if v == x - 1 {
        v += 4;
      }
      ans[i] = v;
      v += 1;
    }
    for i in n + 2 .. 2 * n - 1 {
      if v == x - 1 {
        v += 4;
      }
      ans[i] = v;
      v += 1;
    }
  } else {
    ans[n-2] = x - 1;
    ans[n-1] = x;
    ans[n] = x + 1;
    ans[n+1] = x - 2;
    let mut v = 1;
    for i in 0 .. n - 2 {
      if v == x - 2 {
        v += 4;
      }
      ans[i] = v;
      v += 1;
    }
    for i in n + 2 .. 2 * n - 1 {
      if v == x - 2 {
        v += 4;
      }
      ans[i] = v;
      v += 1;
    }
  }

  for &a in ans.iter() {
    println!("{}", a);
  }
}

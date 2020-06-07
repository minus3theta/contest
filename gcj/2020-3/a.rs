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

fn solve(x: &Vec<char>, y: &Vec<char>) -> String {
  let xl = x.len();
  let yl = y.len();
  let mut dp = vec![vec![1 << 30; yl+1]; xl+1];
  for j in 0 .. yl+1 {
    dp[0][j] = j as i32;
  }
  for i in 0 .. xl+1 {
    dp[i][0] = i as i32;
  }
  for i in 0 .. xl {
    for j in 0 .. yl {
      dp[i+1][j+1] = cmp::min(
        cmp::min(dp[i][j+1] + 1, dp[i+1][j] + 1),
        dp[i][j] + if x[i] == y[j] { 0 } else { 1 }
      );
    }
  }
  let ed = dp[xl][yl];
  let mut i = xl;
  let mut j = yl;
  let mut ans = x.clone();
  while dp[i][j] > ed / 2 {
    match (i.checked_sub(1), j.checked_sub(1)) {
      (Some(pi), Some(pj)) if dp[pi][pj] == dp[i][j] && x[pi] == y[pj] => {
        i = pi;
        j = pj;
      }
      (Some(pi), Some(pj)) if dp[pi][pj] + 1 == dp[i][j] && x[pi] != y[pj] => {
        i = pi;
        j = pj;
        ans[pi] = y[pj];
      }
      (Some(pi), _) if dp[pi][j] + 1 == dp[i][j] => {
        i = pi;
        ans.remove(pi);
      }
      (_, Some(pj)) if dp[i][pj] + 1 == dp[i][j] => {
        j = pj;
        ans.insert(i, y[pj]);
      }
      _ => unreachable!()
    }
  }
  ans.into_iter().collect()
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    t: usize,
    cj: [(chars, chars); t],
  }
  for (t, &(ref c, ref j)) in cj.iter().enumerate() {
    let n = solve(c, j);
    puts!("Case #{}: {}\n", t+1, n);
  }
}

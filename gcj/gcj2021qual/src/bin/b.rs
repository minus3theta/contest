// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
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

fn is_c(c: char) -> bool {
    c == 'C' || c == '?'
}

fn is_j(c: char) -> bool {
    c == 'J' || c == '?'
}

fn solve(x: i64, y: i64, s: &[char]) -> i64 {
    let inf = 1i64 << 60;
    let n = s.len();
    // 0: last is C
    // 1: last is J
    let mut dp = vec![[inf; 2]; n];
    for (i, &c) in s.iter().enumerate() {
        if let Some(j) = i.checked_sub(1) {
            if is_c(c) {
                dp[i][0] = dp[j][0].min(dp[j][1] + y);
            }
            if is_j(c) {
                dp[i][1] = dp[j][1].min(dp[j][0] + x);
            }
        } else {
            if is_c(c) {
                dp[i][0] = 0;
            }
            if is_j(c) {
                dp[i][1] = 0;
            }
        }
    }

    dp[n - 1][0].min(dp[n - 1][1])
}

fn main() {
    let s = {
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };
    let mut iter = s.split_whitespace();
    input_inner! {
        iter,
        t: usize,
    }
    for c in 1..=t {
        input_inner! {
            iter,
            x: i64,
            y: i64,
            s: chars,
        }

        println!("Case #{}: {}", c, solve(x, y, &s));
    }
}

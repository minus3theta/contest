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

type Ret = (i128, i128, i128, i128);
const CYCLE: i128 = 360 * 12 * 10_000_000_000;
const INV: i128 = 30183740042989;

fn solve_hms(h: i128, m: i128, s: i128) -> Option<i128> {
    let rel_m = (m - h + CYCLE) % CYCLE;
    let rel_s = (s - h + CYCLE) % CYCLE;
    let a = 719 * rel_m % CYCLE;
    let b = 11 * rel_s % CYCLE;
    if a != b {
        return None;
    }
    // let (g, x, _) = ext_gcd(719 * 11, CYCLE);
    // dbg!(g);
    Some((a * INV).rem_euclid(CYCLE))
}

fn solve(a: i128, b: i128, c: i128) -> Ret {
    let mut x = solve_hms(a, b, c)
        .or_else(|| solve_hms(a, c, b))
        .or_else(|| solve_hms(b, a, c))
        .or_else(|| solve_hms(b, c, a))
        .or_else(|| solve_hms(c, a, b))
        .or_else(|| solve_hms(c, b, a))
        .unwrap();
    let n = x % 1_000_000_000;
    x /= 1_000_000_000;
    let s = x % 60;
    x /= 60;
    let m = x % 60;
    x /= 60;
    let h = x;
    (h, m, s, n)
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
    for cs in 1..=t {
        input_inner! {
            iter,
            abc: (i128, i128, i128),
        }
        let (a, b, c) = abc;
        let (h, m, s, n) = solve(a, b, c);
        println!("Case #{}: {} {} {} {}", cs, h, m, s, n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1_1() {
        solve(0, 0, 0);
    }

    #[test]
    fn case_1_2() {
        solve(0, 21600000000000, 23400000000000);
    }

    #[test]
    fn case_1_3() {
        solve(1476000000000, 2160000000000, 3723000000000);
    }

    #[test]
    fn case_3_1() {
        solve(0, 11, 719);
    }
}

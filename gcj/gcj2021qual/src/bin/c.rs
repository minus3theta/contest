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

fn cost_ub(n: usize) -> usize {
    n * (n + 1) / 2 - 1
}

fn solve(n: usize, c: usize, offset: usize) -> Option<Vec<usize>> {
    if c < n - 1 || c > cost_ub(n) {
        return None;
    }
    if n == 1 {
        return Some(vec![offset]);
    }
    let next_cost_ub = cost_ub(n - 1);
    let current_cost = c.saturating_sub(next_cost_ub).max(1);
    assert!(current_cost >= 1);
    assert!(current_cost <= n);
    let mut next = solve(n - 1, c - current_cost, offset + 1)?;
    next.insert(0, offset);
    next[0..current_cost].reverse();
    Some(next)
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
            n: usize,
            x: usize,
        }

        let ans = solve(n, x, 1);

        print!("Case #{}: ", c);
        if let Some(ans) = ans {
            for &x in &ans {
                print!("{} ", x);
            }
        } else {
            print!("IMPOSSIBLE");
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cost(l: &mut [usize]) -> usize {
        let n = l.len();
        let mut cost = 0;
        for i in 0..(n - 1) {
            let (j, _) = l
                .iter()
                .enumerate()
                .skip(i)
                .find(|(_, &x)| x == i + 1)
                .unwrap();
            cost += j - i + 1;
            l[i..=j].reverse();
        }
        cost
    }

    #[test]
    fn case0() {
        assert!(solve(1, 0, 1).is_some());
    }
    #[test]
    fn case1() {
        assert!(solve(4, 6, 1).is_some());
    }
    #[test]
    fn case2() {
        assert!(solve(2, 1, 1).is_some());
    }
    #[test]
    fn case3() {
        assert!(solve(7, 12, 1).is_some());
    }
    #[test]
    fn case_max() {
        assert!(solve(2, 2, 1).is_some());
        assert!(solve(3, 5, 1).is_some());
    }
    #[test]
    fn all_cases() {
        for n in 1..100 {
            for c in 1..100 {
                if let Some(mut v) = solve(n, c, 1) {
                    dbg!(&v);
                    assert_eq!(cost(&mut v), c);
                }
            }
        }
    }
}

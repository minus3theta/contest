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

fn feasible(a: usize, b: usize, u: &[i64], x: usize) -> bool {
    if x < u.len() {
        return false;
    }
    let mut v = vec![0; x];
    v[x - 1] = 1;
    for i in (1..x).rev() {
        let rest = *u.get(i).unwrap_or(&0);
        if rest > v[i] {
            return false;
        }
        let conv = v[i] - rest;
        v[i] -= conv;
        if let Some(ia) = i.checked_sub(a) {
            v[ia] += conv;
        }
        if let Some(ib) = i.checked_sub(b) {
            v[ib] += conv;
        }
    }
    (0..u.len()).all(|i| u[i] <= v[i])
}

use std::collections::BTreeSet;

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn solve(a: usize, b: usize, u: &[i64]) -> Option<usize> {
    let mut diffs = BTreeSet::new();
    for i in 0..u.len() {
        for j in i + 1..u.len() {
            if u[i] > 0 && u[j] > 0 {
                diffs.insert(j - i);
            }
        }
    }
    let g = gcd(a, b);
    if diffs.iter().any(|&d| d % g != 0) {
        return None;
    }

    (1usize..).find(|&i| feasible(a, b, u, i))
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
            n: usize,
            a: usize,
            b: usize,
            u: [i64; n],
        }
        if let Some(ans) = solve(a, b, &u) {
            println!("Case #{}: {}", cs, ans);
        } else {
            println!("Case #{}: IMPOSSIBLE", cs);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1_1() {
        assert_eq!(solve(1, 2, &[1, 2]), Some(4));
    }

    #[test]
    fn case_1_2() {
        assert_eq!(solve(1, 2, &[2, 0, 0, 0, 1]), Some(6));
    }

    #[test]
    fn case_1_3() {
        assert_eq!(solve(1, 2, &[1, 1, 1]), Some(5));
    }

    #[test]
    fn case_2_1() {
        assert_eq!(solve(2, 4, &[1, 1, 1]), None);
    }

    #[test]
    fn case_2_2() {
        assert_eq!(solve(2, 4, &[1, 0, 1]), Some(5));
    }

    #[test]
    fn case_2_3() {
        assert_eq!(solve(2, 5, &[1, 0, 0, 0, 1]), Some(10));
    }
}

use either::Either;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

struct Solve {
    n: usize,
    a: Vec<i64>,
}

impl Solve {
    fn new(n: usize, a: Vec<i64>) -> Self {
        Self { n, a }
    }

    fn index(&self, i: usize, j: usize) -> usize {
        assert!(i < j);
        i * (2 * self.n - 1 + 2 * self.n - i) / 2 + j - i - 1
    }

    fn cost(&self, i: usize, j: usize) -> i64 {
        self.a[self.index(i, j)]
    }

    fn dfs(&self, used: u32) -> Vec<i64> {
        if used == 0 {
            return vec![0];
        }
        let i = used.trailing_zeros() as usize;
        (i + 1..self.n * 2)
            .flat_map(|j| {
                if (used >> j) & 1 == 0 {
                    Either::Left(None)
                } else {
                    Either::Right(self.dfs(used ^ (1 << i) ^ (1 << j)))
                }
                .into_iter()
                .map(move |c| c ^ self.cost(i, j))
            })
            .collect()
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n * (2 * n - 1)],
    }
    let solve = Solve::new(n, a);

    println!("{}", solve.dfs((1 << (2 * n)) - 1).iter().max().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let solve = Solve::new(2, vec![]);
        assert_eq!(solve.index(0, 1), 0);
        assert_eq!(solve.index(0, 2), 1);
        assert_eq!(solve.index(0, 3), 2);
        assert_eq!(solve.index(1, 2), 3);
        assert_eq!(solve.index(1, 3), 4);
        assert_eq!(solve.index(2, 3), 5);
    }
}

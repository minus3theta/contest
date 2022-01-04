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
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

trait Then {
    fn then<T>(self, f: impl FnOnce() -> T) -> Option<T>;
}

impl Then for bool {
    fn then<T>(self, f: impl FnOnce() -> T) -> Option<T> {
        if self {
            Some(f())
        } else {
            None
        }
    }
}

fn solve(n: usize, con: &[(usize, usize)]) -> Option<Vec<usize>> {
    let mut lock = vec![0; n];
    let mut release = vec![vec![]; n];
    for &(a, b) in con {
        lock[b] += 1;
        release[a].push(b);
    }
    let mut que = lock
        .iter()
        .enumerate()
        .filter_map(|(i, &l)| (l == 0).then(|| Reverse(i)))
        .collect::<BinaryHeap<_>>();
    let mut ans = vec![];
    while let Some(Reverse(i)) = que.pop() {
        ans.push(i);
        for &l in &release[i] {
            lock[l] -= 1;
            if lock[l] == 0 {
                que.push(Reverse(l));
            }
        }
    }
    (ans.len() == n).then(|| ans)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        con: [(Usize1, Usize1); m],
    }
    if let Some(ans) = solve(n, &con) {
        for a in ans {
            print!("{} ", a + 1);
        }
        println!();
    } else {
        println!("-1");
    }
}

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

const INF: i64 = 1 << 60;

fn main() {
    input! {
        n: usize,
        a: [[i64; n]; n],
        m: usize,
        es: [(Usize1, Usize1); m],
    }
    let mut bad = vec![vec![false; n]; n];
    for &(x, y) in &es {
        bad[x][y] = true;
        bad[y][x] = true;
    }
    let mut ans = INF;
    'outer: for pat in (0..n).permutations(n) {
        for i in 0..n - 1 {
            if bad[pat[i]][pat[i + 1]] {
                continue 'outer;
            }
        }
        ans = ans.min(pat.into_iter().enumerate().map(|(i, p)| a[p][i]).sum());
    }

    println!("{}", if ans == INF { -1 } else { ans });
}

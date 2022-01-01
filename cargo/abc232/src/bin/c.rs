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

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        e0: [(Usize1, Usize1); m],
        e1: [(Usize1, Usize1); m],
    }
    fn to_adj(n: usize, edges: &[(usize, usize)]) -> Vec<Vec<bool>> {
        let mut ret = vec![vec![false; n]; n];
        for &(a, b) in edges {
            ret[a][b] = true;
            ret[b][a] = true;
        }
        ret
    }
    let adj0 = to_adj(n, &e0);
    let adj1 = to_adj(n, &e1);
    let ans = (0..n).permutations(n).any(|perm| {
        (0..n)
            .tuple_combinations()
            .all(|(a, b)| adj0[a][b] == adj1[perm[a]][perm[b]])
    });

    println!("{}", if ans { "Yes" } else { "No" });
}

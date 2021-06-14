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
        q: usize,
        es: [(Usize1, Usize1); m],
        qs: [(Usize1, Usize1); q],
    }
    let mut income = vec![vec![]; n];
    for &(u, v) in &es {
        income[v].push(u);
    }
    for chunk in qs.chunks(64) {
        let mut reachable = vec![0u64; n];
        for (i, &(a, _)) in chunk.iter().enumerate() {
            reachable[a] |= 1 << i;
        }
        for v in 0..n {
            for &u in &income[v] {
                reachable[v] |= reachable[u];
            }
        }
        for (i, &(_, b)) in chunk.iter().enumerate() {
            println!(
                "{}",
                if (reachable[b] >> i) & 1 != 0 {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}

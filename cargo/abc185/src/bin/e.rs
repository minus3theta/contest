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

const INF: i32 = 1 << 30;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; m],
    }
    let mut score = vec![vec![INF; m + 1]; n + 1];
    score[0][0] = 0;
    for i in 0..=n {
        let pi = i.checked_sub(1);
        for j in 0..=m {
            let pj = j.checked_sub(1);
            if let Some(pi) = pi {
                score[i][j] = cmp::min(score[i][j], score[pi][j] + 1);
                if let Some(pj) = pj {
                    let penalty = if a[pi] == b[pj] { 0 } else { 1 };
                    score[i][j] = cmp::min(score[i][j], score[pi][pj] + penalty);
                }
            }
            if let Some(pj) = pj {
                score[i][j] = cmp::min(score[i][j], score[i][pj] + 1);
            }
        }
    }

    println!("{}", score[n][m]);
}

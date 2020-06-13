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

const W: usize = 100_000;
// const W: usize = 10;

#[fastout]
fn main() {
    input! {
        n: usize,
        item: [(i64, usize); n],
        q: usize,
        query: [(Usize1, usize); q],
    }
    let m = cmp::min(n, 1024) as usize;
    let mut max_val = vec![vec![0i64; W + 1]; m + 1];
    for (i, &(a, b)) in item.iter().take(m).enumerate() {
        for w in 0..=W {
            max_val[i + 1][w] =
                cmp::max(max_val[(i + 1) / 2][w], max_val[i + 1][w.saturating_sub(1)]);
            if let Some(pw) = w.checked_sub(b) {
                max_val[i + 1][w] = cmp::max(max_val[i + 1][w], max_val[(i + 1) / 2][pw] + a);
            }
        }
    }
    for &(v, l) in &query {
        if v < m {
            println!("{}", max_val[v + 1][l]);
        } else {
            let mut vs = vec![item[v]];
            let mut v = v;
            loop {
                v = (v - 1) / 2;
                if v < m {
                    break;
                }
                vs.push(item[v]);
            }
            let last_v = v;
            let mut ret = 0i64;
            for pat in 0..1 << vs.len() {
                let mut value = 0i64;
                let mut w = 0usize;
                for (i, &(a, b)) in vs.iter().enumerate() {
                    if (pat >> i) & 1 != 0 {
                        value += a;
                        w += b;
                    }
                }

                if let Some(pw) = l.checked_sub(w) {
                    ret = cmp::max(ret, value + max_val[last_v + 1][pw]);
                }
            }
            println!("{}", ret);
        }
    }
}

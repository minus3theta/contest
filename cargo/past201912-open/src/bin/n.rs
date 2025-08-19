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
        w: i64,
        c: i64,
        stones: [(i64, i64, i64); n],
    }
    let mut map = BTreeMap::new();
    let mut sum = 0;
    for (x, p) in stones
        .into_iter()
        .flat_map(|(l, r, p)| [(l - c, p), (r - 1, -p)])
    {
        if x < 0 {
            sum += p;
            continue;
        }
        *map.entry(x).or_insert(0) += p;
    }
    let mut ans = sum;
    for (&x, &p) in &map {
        if x >= w - c {
            break;
        }
        sum += p;
        ans = ans.min(sum);
    }

    println!("{}", ans);
}

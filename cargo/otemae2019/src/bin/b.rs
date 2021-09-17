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
        m: usize,
        n: usize,
        k: usize,
        xs: [Usize1; n],
    }

    let xs = {
        let mut mp = BTreeMap::<usize, usize>::new();
        for x in xs {
            *mp.entry(x).or_insert(0) += 1;
        }
        mp
    };

    let solve = |p: usize| {
        (1..=k)
            .filter(|&i| {
                p.checked_sub(i)
                    .map_or(false, |dist| xs.contains_key(&dist))
                    || xs.contains_key(&(p + i))
            })
            .count()
            + xs.get(&p).unwrap_or(&0)
    };
    let ans = (0..m).map(solve).max().unwrap();

    println!("{}", ans);
}

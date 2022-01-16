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

#[fastout]
fn main() {
    input! {
        n: usize,
        ss: [i64; n],
        tt: [i64; n],
    }
    let mut que = tt
        .into_iter()
        .enumerate()
        .map(|(i, t)| Reverse((t, i)))
        .collect::<BinaryHeap<_>>();
    let mut ans = vec![None; n];
    while let Some(Reverse((t, i))) = que.pop() {
        if ans[i].is_some() {
            continue;
        }
        ans[i] = Some(t);
        que.push(Reverse((t + ss[i], (i + 1) % n)));
    }

    for a in ans {
        println!("{}", a.unwrap());
    }
}

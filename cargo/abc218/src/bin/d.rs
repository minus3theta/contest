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
        point: [(i64, i64); n],
    }
    let point = point.into_iter().collect::<BTreeSet<_>>();
    let mut ans = 0;
    for (&p, &q) in point.iter().tuple_combinations() {
        if p.0 < q.0 && p.1 < q.1 && point.contains(&(p.0, q.1)) && point.contains(&(q.0, p.1)) {
            ans += 1;
        }
    }

    println!("{}", ans);
}

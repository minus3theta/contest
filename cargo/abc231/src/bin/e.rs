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

fn solve(a: &[i64], memo: &mut HashMap<(i64, usize), i64>, x: i64, i: usize) -> i64 {
    if let Some(&v) = memo.get(&(x, i)) {
        return v;
    }
    let v = calc(a, memo, x, i);
    memo.insert((x, i), v);
    v
}

fn calc(aa: &[i64], memo: &mut HashMap<(i64, usize), i64>, x: i64, i: usize) -> i64 {
    let a = aa[i];
    if i == 0 {
        return x / a;
    }
    let less = x / a + solve(aa, memo, x - (x / a * a), i - 1);
    let more = x / a + 1 + solve(aa, memo, ((x / a + 1) * a) - x, i - 1);
    less.min(more)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        x: i64,
        a: [i64; n],
    }

    println!("{}", solve(&a, &mut HashMap::new(), x, n - 1));
}

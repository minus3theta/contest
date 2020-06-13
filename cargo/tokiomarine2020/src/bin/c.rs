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

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut a: [usize; n]
    }
    for _ in 0..k {
        if a.iter().all(|&x| x == n) {
            break;
        }
        let mut sum = vec![0; n + 1];
        for i in 0..n {
            sum[i.saturating_sub(a[i])] += 1;
            sum[cmp::min(i + a[i] + 1, n)] -= 1;
        }
        for i in 0..n {
            sum[i + 1] += sum[i];
            a[i] = sum[i] as usize;
        }
    }

    for &x in &a {
        print!("{} ", x);
    }
    println!();
}

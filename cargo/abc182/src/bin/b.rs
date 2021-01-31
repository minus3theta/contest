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
        a: [i32; n],
    }
    let mut ans = 0;
    let mut count = 0;
    for i in 2..=*a.iter().max().unwrap() {
        let c = a.iter().filter(|&x| x % i == 0).count();
        if c > count {
            count = c;
            ans = i;
        }
    }

    println!("{}", ans);
}

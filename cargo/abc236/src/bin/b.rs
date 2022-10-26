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
        a: [usize; 4 * n - 1],
    }
    let mut popl = vec![0; n + 1];
    for &x in &a {
        popl[x] += 1;
    }
    let (ans, _) = popl.iter().find_position(|&&p| p == 3).unwrap();

    println!("{}", ans);
}

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
        a: [Usize1; n],
    }
    let mut popl = vec![0; n];
    for &a in &a {
        popl[a] += 1;
    }
    if let (Some((p0, _)), Some((p2, _))) = (
        popl.iter().find_position(|&&p| p == 0),
        popl.iter().find_position(|&&p| p == 2),
    ) {
        println!("{} {}", p2 + 1, p0 + 1);
    } else {
        println!("Correct");
    }
}

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
        s: Chars,
    }
    let mut popl = [0; 3];
    for &c in &s {
        match c {
            'a' => popl[0] += 1,
            'b' => popl[1] += 1,
            'c' => popl[2] += 1,
            _ => unreachable!(),
        }
    }
    if popl[0] > popl[1] && popl[0] > popl[2] {
        println!("a");
    } else if popl[1] > popl[0] && popl[1] > popl[2] {
        println!("b");
    } else {
        println!("c");
    }
}

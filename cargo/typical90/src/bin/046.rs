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

const MOD: usize = 46;

fn popl(v: &[usize]) -> Vec<i64> {
    let mut p = vec![0; MOD];
    for &x in v {
        p[x % MOD] += 1;
    }
    p
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }
    let a_popl = popl(&a);
    let b_popl = popl(&b);
    let c_popl = popl(&c);
    let mut ans = 0;
    for (i, &ap) in a_popl.iter().enumerate() {
        for (j, &bp) in b_popl.iter().enumerate() {
            let k = (MOD * 2 - i - j) % MOD;
            ans += ap * bp * c_popl[k];
        }
    }
    println!("{}", ans);
}

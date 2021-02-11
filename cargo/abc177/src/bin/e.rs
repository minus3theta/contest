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
        a: [usize; n],
    }
    let max_a = *a.iter().max().unwrap();
    let mut divisor = vec![0; max_a + 1];
    for p in 2..=max_a {
        if divisor[p] != 0 {
            continue;
        }
        divisor[p] = p;
        for i in 2.. {
            if p * i > max_a {
                break;
            }
            divisor[p * i] = p;
        }
    }
    let mut popl = HashMap::new();
    for &x in &a {
        if x == 1 {
            continue;
        }
        let mut factor = HashSet::new();
        let mut d = x;
        while divisor[d] != d {
            factor.insert(divisor[d]);
            d /= divisor[d];
        }
        factor.insert(d);
        for &f in &factor {
            *popl.entry(f).or_insert(0) += 1;
        }
    }
    if popl.values().all(|&x| x == 1) {
        println!("pairwise coprime");
    } else if popl.values().any(|&x| x == n) {
        println!("not coprime");
    } else {
        println!("setwise coprime");
    }
}

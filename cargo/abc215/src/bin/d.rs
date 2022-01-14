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
        m: usize,
        aa: [usize; n],
    }
    let mut factors = BTreeSet::new();
    for &a in &aa {
        let mut a = a;
        for i in 2.. {
            if i * i > a {
                break;
            }
            if a % i == 0 {
                factors.insert(i);
                while a % i == 0 {
                    a /= i;
                }
            }
        }
        if a != 1 {
            factors.insert(a);
        }
    }
    let mut ok = vec![true; m + 1];
    ok[0] = false;
    for &f in &factors {
        let mut x = f;
        while x <= m {
            ok[x] = false;
            x += f;
        }
    }
    let ans = ok
        .iter()
        .enumerate()
        .filter_map(|(i, &o)| if o { Some(i) } else { None })
        .collect_vec();
    println!("{}", ans.len());
    for &i in &ans {
        println!("{}", i);
    }
}

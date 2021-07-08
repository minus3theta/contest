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
        k: i64,
    }
    let mut divisors = vec![];
    for i in 1.. {
        if i * i > k {
            break;
        }
        let (q, r) = k.div_rem(&i);
        if r != 0 {
            continue;
        }
        divisors.push(i);
        if q != i {
            divisors.push(q);
        }
    }
    divisors.sort();
    let mut ans = 0;
    for ab in divisors.into_iter().combinations_with_replacement(2) {
        let a = ab[0];
        let b = ab[1];
        let q = k / a / b;
        if b <= q {
            let r = k - a * b * q;
            if r == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}

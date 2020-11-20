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
        q: usize,
        hh: [i64; n],
    }
    let mut eo_diff = BTreeMap::new();
    let mut oe_diff = BTreeMap::new();
    let mut diff = vec![0; n - 1];
    for i in 0..n - 1 {
        diff[i] = hh[i + 1] - hh[i];
        if i % 2 == 0 {
            *eo_diff.entry(diff[i]).or_insert(0) += 1;
        } else {
            *oe_diff.entry(diff[i]).or_insert(0) += 1;
        }
    }
    // dbg!(&eo_diff);
    // dbg!(&oe_diff);
    // odd - even
    let mut offset = 0;
    for _ in 0..q {
        input! {
            t: i32,
        }
        match t {
            1 => {
                // even
                input! {
                    v: i64,
                }
                offset += v;
            }
            2 => {
                // odd
                input! {
                    v: i64,
                }
                offset -= v;
            }
            _ => {
                input! {
                    u: Usize1,
                    v: i64,
                }
                if u < n - 1 {
                    if u % 2 == 0 {
                        *eo_diff.get_mut(&diff[u]).unwrap() -= 1;
                        *eo_diff.entry(diff[u] - v).or_insert(0) += 1;
                    } else {
                        *oe_diff.get_mut(&diff[u]).unwrap() -= 1;
                        *oe_diff.entry(diff[u] - v).or_insert(0) += 1;
                    }
                    diff[u] -= v;
                }
                if u > 0 {
                    let u = u - 1;
                    if u % 2 == 0 {
                        *eo_diff.get_mut(&diff[u]).unwrap() -= 1;
                        *eo_diff.entry(diff[u] + v).or_insert(0) += 1;
                    } else {
                        *oe_diff.get_mut(&diff[u]).unwrap() -= 1;
                        *oe_diff.entry(diff[u] + v).or_insert(0) += 1;
                    }
                    diff[u] += v;
                }
            }
        }
        // dbg!(&eo_diff);
        // dbg!(&oe_diff);
        let mut ans = 0;
        ans += eo_diff.get(&offset).unwrap_or(&0);
        ans += oe_diff.get(&-offset).unwrap_or(&0);
        println!("{}", ans);
    }
}

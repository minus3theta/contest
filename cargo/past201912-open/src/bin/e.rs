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
    }
    let mut follow = vec![vec![false; n]; n];
    for _ in 0..q {
        input! {
            t: i32,
        }
        match t {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                follow[a][b] = true;
            }
            2 => {
                input! {
                    a: Usize1,
                }
                for b in 0..n {
                    if follow[b][a] {
                        follow[a][b] = true;
                    }
                }
            }
            _ => {
                input! {
                    a: Usize1,
                }
                let following = follow[a]
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &f)| if f { Some(i) } else { None })
                    .collect::<Vec<_>>();
                for &b in &following {
                    for c in 0..n {
                        if c != a && follow[b][c] {
                            follow[a][c] = true;
                        }
                    }
                }
            }
        }
    }

    for fol in &follow {
        for &f in fol {
            if f {
                print!("Y");
            } else {
                print!("N");
            }
        }
        println!();
    }
}

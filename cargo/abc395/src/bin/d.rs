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
    let mut id_to_label = (0..n).collect::<Vec<_>>();
    let mut label_to_id = (0..n).collect::<Vec<_>>();
    let mut p_to_id = (0..n).collect::<Vec<_>>();
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
                p_to_id[a] = label_to_id[b];
            }
            2 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                }
                id_to_label.swap(label_to_id[a], label_to_id[b]);
                label_to_id.swap(a, b);
            }
            3 => {
                input! {
                    a: Usize1,
                }
                println!("{}", id_to_label[p_to_id[a]] + 1);
            }
            _ => unreachable!(),
        }
    }
}

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
        q: usize,
    }
    let mut s = VecDeque::new();
    for _ in 0..q {
        input! {
            t: i32,
        }
        match t {
            1 => {
                input! {
                    c: char,
                    x: i64,
                }
                s.push_back((c, x));
            }
            _ => {
                input! {
                    mut d: i64,
                }
                let mut removed = BTreeMap::new();
                while d > 0 {
                    if let Some((c, x)) = s.pop_front() {
                        if x <= d {
                            d -= x;
                            *removed.entry(c).or_insert(0) += x;
                        } else {
                            s.push_front((c, x - d));
                            *removed.entry(c).or_insert(0) += d;
                            d = 0;
                        }
                    } else {
                        break;
                    }
                }
                let score = removed.values().map(|&x| x * x).sum::<i64>();
                println!("{}", score);
            }
        }
    }
}

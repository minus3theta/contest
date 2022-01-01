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
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

fn compress<T>(array: &[T]) -> BTreeMap<T, usize>
where
    T: Clone + PartialEq + Ord,
{
    let mut array = array.to_vec();
    array.sort();
    array.dedup();
    array.into_iter().enumerate().map(|(i, a)| (a, i)).collect()
}

use bit::*;
pub mod bit {
    /// 1-indexed
    /// example: Bit::new(n, |x, y| *x += *y, 0)
    pub struct Bit<T, F> {
        n: usize,
        dat: Vec<T>,
        op: F,
        unit: T,
    }
    impl<T: Clone, F: Fn(&mut T, &T)> Bit<T, F> {
        pub fn new(n: usize, op: F, unit: T) -> Self {
            Bit {
                n,
                dat: vec![unit.clone(); n + 1],
                op,
                unit,
            }
        }
        pub fn from_vec(mut v: Vec<T>, op: F, unit: T) -> Self {
            let n = v.len();
            let mut dat = vec![unit.clone()];
            dat.append(&mut v);
            for i in 1..n {
                let b = {
                    let i = i as i32;
                    (i & -i) as usize
                };
                let x = dat[i].clone();
                if i + b <= n {
                    op(&mut dat[i + b], &x);
                }
            }
            Bit { n, dat, op, unit }
        }
        pub fn operate(&mut self, k: usize, a: &T) {
            let mut k = k;
            while k <= self.n {
                (self.op)(&mut self.dat[k], a);
                let l = k as i32;
                k += (l & -l) as usize;
            }
        }
        pub fn accum(&self, k: usize) -> T {
            let mut k = k;
            let mut sum = self.unit.clone();
            while k > 0 {
                (self.op)(&mut sum, &self.dat[k]);
                let l = k as i32;
                k -= (l & -l) as usize;
            }
            sum
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [i64; n],
        b: [i64; n],
    }
    let b_comp = compress(&b);
    let mut p = BTreeMap::new();
    for (a, b) in a.into_iter().zip(b) {
        *p.entry((Reverse(a), b)).or_insert(0) += 1;
    }
    let mut bit = Bit::new(b_comp.len(), |x, y| *x += *y, 0);
    let mut ans = 0i64;
    for ((_, b), count) in p.into_iter() {
        bit.operate(b_comp[&b] + 1, &count);
        ans += bit.accum(b_comp[&b] + 1) * count;
    }

    println!("{}", ans);
}

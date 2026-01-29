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
use bit::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
    }
    let mut sum = vec![(0i64, cmp::Reverse(0)); n + 1];
    for (i, &c) in s.iter().enumerate() {
        let d = match c {
            'A' => 1,
            'B' => -1,
            _ => 0,
        };
        sum[i + 1] = (sum[i].0 + d, cmp::Reverse(i + 1));
    }
    sum.sort();
    let mut tree = Bit::new(n + 1, |x, y| *x += *y, 0i64);
    let mut ans = 0;
    for &(_, cmp::Reverse(i)) in &sum {
        ans += tree.accum(i + 1);
        tree.operate(i + 1, &1);
    }

    println!("{ans}");
}

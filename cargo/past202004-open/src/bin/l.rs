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

use segtree::*;
pub mod segtree {
    pub struct Segtree<T, F> {
        n: usize,
        dat: Vec<T>,
        op: F,
        unit: T,
    }
    impl<T: Clone, F: Fn(&T, &T) -> T> Segtree<T, F> {
        pub fn new(n: usize, op: F, unit: T) -> Self {
            Segtree {
                n: n.next_power_of_two(),
                dat: vec![unit.clone(); 2 * n.next_power_of_two() - 1],
                op,
                unit,
            }
        }
        #[allow(clippy::manual_repeat_n)]
        pub fn from_vec(v: Vec<T>, op: F, unit: T) -> Self {
            use std::iter::repeat;
            let n = v.len();
            let base = n.next_power_of_two();
            let mut dat: Vec<_> = repeat(unit.clone())
                .take(base - 1)
                .chain(v)
                .chain(repeat(unit.clone()).take(base - n))
                .collect();
            assert_eq!(dat.len(), 2 * base - 1);
            for i in (0..base - 1).rev() {
                dat[i] = op(&dat[i * 2 + 1], &dat[i * 2 + 2]);
            }
            Segtree {
                n: base,
                dat,
                op,
                unit,
            }
        }
        pub fn update(&mut self, mut k: usize, x: T) {
            k += self.n - 1;
            self.dat[k] = x;
            while k > 0 {
                k = (k - 1) / 2;
                self.dat[k] = (self.op)(&self.dat[k * 2 + 1], &self.dat[k * 2 + 2]);
            }
        }
        pub fn accum(&self, a: usize, b: usize) -> T {
            self.accum_inner(a, b, 0, 0, self.n)
        }
        fn accum_inner(&self, a: usize, b: usize, k: usize, left: usize, right: usize) -> T {
            if right <= a || b <= left {
                return self.unit.clone();
            }
            if a <= left && right <= b {
                return self.dat[k].clone();
            }
            let vl = self.accum_inner(a, b, k * 2 + 1, left, (left + right) / 2);
            let vr = self.accum_inner(a, b, k * 2 + 2, (left + right) / 2, right);
            (self.op)(&vl, &vr)
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [i64; n],
    }
    if d * (k - 1) + 1 > n {
        println!("-1");
        return;
    }
    let a = a
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<Vec<_>>();
    let tree = Segtree::from_vec(a, |&x, &y| x.min(y), (1_i64 << 60, 0));
    let mut left = 0;
    let mut right = n - d * (k - 1);
    for _ in 0..k {
        let (x, i) = tree.accum(left, right);
        print!("{x} ");
        left = i + d;
        right += d;
    }

    println!();
}

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

// 1-indexed
// example: Bit::new(n, |x, y| *x += *y, 0)
struct Bit<T, F> {
    n: usize,
    dat: Vec<T>,
    op: F,
    unit: T,
}

impl<T: Clone, F: Fn(&mut T, &T)> Bit<T, F> {
    #[allow(dead_code)]
    fn new(n: usize, op: F, unit: T) -> Self {
        Bit {
            n: n,
            dat: vec![unit.clone(); n + 1],
            op: op,
            unit: unit,
        }
    }
    #[allow(dead_code)]
    fn from_vec(mut v: Vec<T>, op: F, unit: T) -> Self {
        let n = v.len();
        let mut dat = vec![unit.clone()];
        dat.append(&mut v);
        for i in 1..n {
            let j = i as i32;
            let b = (j & -j) as usize;
            let x = dat[i].clone();
            op(&mut dat[i + b], &x);
        }
        Bit {
            n: n,
            dat: dat,
            op: op,
            unit: unit,
        }
    }
    fn operate(&mut self, k: usize, a: &T) {
        let mut k = k;
        while k <= self.n {
            (self.op)(&mut self.dat[k], &a);
            let l = k as i32;
            k += (l & -l) as usize;
        }
    }
    fn accum(&self, k: usize) -> T {
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

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        cs: [Usize1; n],
        qs: [(Usize1, Usize1); q],
    }
    let mut qs: Vec<_> = qs.into_iter().enumerate().collect();
    qs.sort_by_key(|&(_, (_, r))| r);

    let mut ans = vec![0; q];
    let mut good = vec![None::<usize>; n];
    let mut bit = Bit::new(n, |x, y| *x += y, 0);

    let mut right = 0;
    for (i, (l, r)) in qs.into_iter() {
        while right <= r {
            let color = cs[right];
            if let Some(prev) = good[color] {
                bit.operate(prev + 1, &-1);
            }
            good[color] = Some(right);
            bit.operate(right + 1, &1);
            right += 1;
        }
        ans[i] = bit.accum(r + 1) - bit.accum(l);
    }

    for &a in &ans {
        println!("{}", a);
    }
}

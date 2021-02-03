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
            if i + b <= n {
                op(&mut dat[i + b], &x);
            }
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

fn block_larger(v: &mut Vec<usize>) {
    let mut blocked = false;
    for x in v {
        if *x == 0 {
            blocked = true;
        }
        if blocked {
            *x = 0;
        }
    }
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        m: usize,
        stone: [(Usize1, Usize1); m],
    }
    let mut rows = vec![w; h];
    let mut cols = vec![h; w];
    for &(r, c) in &stone {
        rows[r] = cmp::min(rows[r], c);
        cols[c] = cmp::min(cols[c], r);
    }
    block_larger(&mut rows);
    block_larger(&mut cols);
    let cols = cols
        .into_iter()
        .enumerate()
        .sorted_by_key(|(_, r)| cmp::Reverse(*r))
        .collect_vec();
    let mut iter = cols.into_iter().peekable();
    let mut bit = Bit::new(w, |x, y| *x += y, 0);
    let mut ans = 0i64;
    for (r, &c) in rows.iter().enumerate().rev() {
        loop {
            if let Some(&(vc, vr)) = iter.peek() {
                if vr > r {
                    bit.operate(vc + 1, &1);
                    iter.next();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        ans += bit.accum(w) + c as i64 - bit.accum(c);
    }

    println!("{}", ans);
}

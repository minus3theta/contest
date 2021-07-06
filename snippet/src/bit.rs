use cargo_snippet::snippet;

#[snippet(prefix = "use bit::*;")]
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
                let j = i as i32;
                let b = (j & -j) as usize;
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
                (self.op)(&mut self.dat[k], &a);
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

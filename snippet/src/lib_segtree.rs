use cargo_snippet::snippet;

#[snippet(prefix = "use segtree::*;")]
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
        pub fn from_vec(v: Vec<T>, op: F, unit: T) -> Self {
            use std::iter::repeat;
            let n = v.len();
            let base = n.next_power_of_two();
            let mut dat: Vec<_> = repeat(unit.clone())
                .take(base - 1)
                .chain(v.into_iter())
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

#[snippet(prefix = "use lazy_segtree::*;")]
pub mod lazy_segtree {
    pub struct LazySegtree<T, F, L, M, C> {
        n: usize,
        dat: Vec<T>,
        op: F,
        unit: T,
        lazy: Vec<L>,
        mapping: M,
        composition: C,
        identity: L,
    }

    impl<T, F, L, M, C> LazySegtree<T, F, L, M, C>
    where
        T: Clone,
        F: Fn(&T, &T) -> T,
        L: Clone + PartialEq,
        M: Fn(&T, &L) -> T,
        C: Fn(&L, &L) -> L,
    {
        pub fn new(n: usize, op: F, unit: T, mapping: M, composition: C, identity: L) -> Self {
            LazySegtree {
                n: n.next_power_of_two(),
                dat: vec![unit.clone(); 2 * n.next_power_of_two() - 1],
                op,
                unit,
                lazy: vec![identity.clone(); 2 * n.next_power_of_two() - 1],
                mapping,
                composition,
                identity,
            }
        }
        pub fn from_vec(
            v: Vec<T>,
            op: F,
            unit: T,
            mapping: M,
            composition: C,
            identity: L,
        ) -> Self {
            use std::iter::repeat;
            let n = v.len();
            let base = n.next_power_of_two();
            let mut dat: Vec<_> = repeat(unit.clone())
                .take(base - 1)
                .chain(v.into_iter())
                .chain(repeat(unit.clone()).take(base - n))
                .collect();
            assert_eq!(dat.len(), 2 * base - 1);
            for i in (0..base - 1).rev() {
                dat[i] = op(&dat[i * 2 + 1], &dat[i * 2 + 2]);
            }
            LazySegtree {
                n: base,
                dat,
                op,
                unit,
                lazy: vec![identity.clone(); base],
                mapping,
                composition,
                identity,
            }
        }
        fn eval(&mut self, k: usize, l: usize, r: usize) {
            if self.lazy[k] != self.identity {
                self.dat[k] = (self.mapping)(&self.dat[k], &self.lazy[k]);
            }
            if r - l > 1 {
                self.lazy[2 * k + 1] = (self.composition)(&self.lazy[2 * k + 1], &self.lazy[k]);
                self.lazy[2 * k + 2] = (self.composition)(&self.lazy[2 * k + 2], &self.lazy[k]);
            }
            self.lazy[k] = self.identity.clone();
        }
        pub fn update(&mut self, a: usize, b: usize, x: &L) {
            self.update_inner(a, b, x, 0, 0, self.n);
        }
        fn update_inner(&mut self, a: usize, b: usize, x: &L, k: usize, left: usize, right: usize) {
            self.eval(k, left, right);
            if b <= left || right <= a {
                return;
            }
            if a <= left && right <= b {
                self.lazy[k] = (self.composition)(&self.lazy[k], x);
                self.eval(k, left, right);
            } else {
                let mid = (left + right) / 2;
                self.update_inner(a, b, x, 2 * k + 1, left, mid);
                self.update_inner(a, b, x, 2 * k + 2, mid, right);
                self.dat[k] = (self.op)(&self.dat[2 * k + 1], &self.dat[2 * k + 2]);
            }
        }
        pub fn accum(&mut self, a: usize, b: usize) -> T {
            self.accum_inner(a, b, 0, 0, self.n)
        }
        fn accum_inner(&mut self, a: usize, b: usize, k: usize, left: usize, right: usize) -> T {
            if right <= a || b <= left {
                return self.unit.clone();
            }
            self.eval(k, left, right);
            if a <= left && right <= b {
                return self.dat[k].clone();
            }
            let vl = self.accum_inner(a, b, k * 2 + 1, left, (left + right) / 2);
            let vr = self.accum_inner(a, b, k * 2 + 2, (left + right) / 2, right);
            (self.op)(&vl, &vr)
        }
    }
}

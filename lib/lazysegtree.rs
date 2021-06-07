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
    pub fn from_vec(v: Vec<T>, op: F, unit: T, mapping: M, composition: C, identity: L) -> Self {
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
    fn update_inner(&mut self, a: usize, b: usize, x: &L, k: usize, l: usize, r: usize) {
        self.eval(k, l, r);
        if b <= l || r <= a {
            return;
        }
        if a <= l && r <= b {
            self.lazy[k] = (self.composition)(&self.lazy[k], x);
            self.eval(k, l, r);
        } else {
            let mid = (l + r) / 2;
            self.update_inner(a, b, x, 2 * k + 1, l, mid);
            self.update_inner(a, b, x, 2 * k + 2, mid, r);
            self.dat[k] = (self.op)(&self.dat[2 * k + 1], &self.dat[2 * k + 2]);
        }
    }
    pub fn accum(&mut self, a: usize, b: usize) -> T {
        self.accum_inner(a, b, 0, 0, self.n)
    }
    fn accum_inner(&mut self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            return self.unit.clone();
        }
        self.eval(k, l, r);
        if a <= l && r <= b {
            return self.dat[k].clone();
        }
        let vl = self.accum_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let vr = self.accum_inner(a, b, k * 2 + 2, (l + r) / 2, r);
        (self.op)(&vl, &vr)
    }
}

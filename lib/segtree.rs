struct Segtree<T, F> {
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
    fn accum_inner(&self, a: usize, b: usize, k: usize, l: usize, r: usize) -> T {
        if r <= a || b <= l {
            return self.unit.clone();
        }
        if a <= l && r <= b {
            return self.dat[k].clone();
        }
        let vl = self.accum_inner(a, b, k * 2 + 1, l, (l + r) / 2);
        let vr = self.accum_inner(a, b, k * 2 + 2, (l + r) / 2, r);
        (self.op)(&vl, &vr)
    }
}

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

use lazy_segtree::*;
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
            use std::iter::repeat_n;

            let n = v.len();
            let base = n.next_power_of_two();
            let mut dat: Vec<_> = repeat_n(unit.clone(), base - 1)
                .chain(v)
                .chain(repeat_n(unit.clone(), base - n))
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
                lazy: vec![identity.clone(); 2 * base - 1],
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

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Elm {
    count: i64,
    open: usize,
    close: usize,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct Act {
    flip: i32,
    put: i64,
}

fn op(x: &Elm, y: &Elm) -> Elm {
    Elm {
        count: x.count.max(y.count),
        open: x.open + y.open,
        close: x.close + y.close,
    }
}

fn mapping(x: &Elm, act: &Act) -> Elm {
    match (*x, *act) {
        (Elm { count, open, .. }, Act { flip: 0, put }) if open > 0 => Elm {
            count: count + put,
            ..*x
        },
        (Elm { open, .. }, Act { flip, put }) if flip % 2 == 0 => Elm {
            count: if open == 0 { 0 } else { put },
            ..*x
        },
        (Elm { open, close, .. }, Act { put, .. }) => Elm {
            count: if close == 0 { 0 } else { put },
            open: close,
            close: open,
        },
    }
}

fn composition(f: &Act, g: &Act) -> Act {
    if g.flip > 0 {
        Act {
            flip: f.flip + g.flip,
            put: g.put,
        }
    } else {
        Act {
            flip: f.flip,
            put: f.put + g.put,
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let init = vec![
        Elm {
            count: 0,
            open: 1,
            close: 0
        };
        n
    ];
    let mut tree = LazySegtree::from_vec(
        init,
        op,
        Elm {
            count: 0,
            open: 0,
            close: 0,
        },
        mapping,
        composition,
        Act::default(),
    );
    for _ in 0..q {
        input! {
            t: i32,
        }
        match t {
            1 => {
                input! {
                    l: Usize1,
                    r: usize,
                    x: i64,
                }
                tree.update(l, r, &Act { flip: 0, put: x });
            }
            2 => {
                input! {
                    l: Usize1,
                    r: usize,
                }
                tree.update(l, r, &Act { flip: 1, put: 0 });
            }
            3 => {
                input! {
                    l: Usize1,
                    r: usize,
                }
                let max = tree.accum(l, r);
                println!("{}", max.count);
            }
            _ => unreachable!(),
        }
    }
}

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

pub mod modint {
    use std::ops::*;
    pub trait Mod: Copy {
        const MOD: i64;
    }
    #[derive(Copy, Clone, Hash, PartialEq, Eq)]
    pub struct ModInt<M> {
        x: i64,
        marker: std::marker::PhantomData<M>,
    }
    impl<M: Mod> ModInt<M> {
        pub fn new(x: i64) -> Self {
            let x = x % M::MOD;
            Self::new_internal(if x < 0 { x + M::MOD } else { x })
        }
        fn new_internal(x: i64) -> Self {
            ModInt {
                x,
                marker: std::marker::PhantomData,
            }
        }
        pub fn pow(self, e: i64) -> Self {
            if e == 0 {
                Self::new(1)
            } else if e % 2 == 1 {
                self.pow(e - 1) * self
            } else {
                let y = self.pow(e / 2);
                y * y
            }
        }
        pub fn inv(self) -> Self {
            self.pow(M::MOD - 2)
        }
        pub fn fact(n: usize) -> Vec<Self> {
            let mut ret = vec![0.into(); n + 1];
            ret[0] = 1.into();
            for i in 1..n + 1 {
                ret[i] = ret[i - 1] * i as i64;
            }
            ret
        }
        pub fn inv_fact(fact: &[Self]) -> Vec<Self> {
            let n = fact.len() - 1;
            let mut ret = vec![0.into(); n + 1];
            ret[n] = fact[n].inv();
            for i in (0..n).rev() {
                ret[i] = ret[i + 1] * (i + 1) as i64;
            }
            ret
        }
        pub fn facts(n: usize) -> (Vec<Self>, Vec<Self>) {
            let fact = Self::fact(n);
            let inv_fact = Self::inv_fact(&fact);
            (fact, inv_fact)
        }
        pub fn comb<'a>(
            fact: &'a [Self],
            inv_fact: &'a [Self],
        ) -> impl Fn(usize, usize) -> Self + 'a {
            move |x, y| {
                if y <= x {
                    fact[x] * inv_fact[y] * inv_fact[x - y]
                } else {
                    0.into()
                }
            }
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Add<T> for ModInt<M> {
        type Output = Self;
        #[allow(clippy::suspicious_arithmetic_impl)]
        fn add(self, other: T) -> Self {
            let other = other.into();
            let sum = self.x + other.x;
            Self::new_internal(if sum >= M::MOD { sum - M::MOD } else { sum })
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
        type Output = Self;
        #[allow(clippy::suspicious_arithmetic_impl)]
        fn sub(self, other: T) -> Self {
            let other = other.into();
            let sum = self.x - other.x;
            Self::new_internal(if sum < 0 { sum + M::MOD } else { sum })
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Mul<T> for ModInt<M> {
        type Output = Self;
        fn mul(self, other: T) -> Self {
            let other = other.into();
            Self::new_internal(self.x * other.x % M::MOD)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Div<T> for ModInt<M> {
        type Output = Self;
        #[allow(clippy::suspicious_arithmetic_impl)]
        fn div(self, other: T) -> Self {
            let other = other.into();
            self * other.inv()
        }
    }
    impl<M: Mod> From<i64> for ModInt<M> {
        fn from(x: i64) -> Self {
            Self::new(x)
        }
    }
    impl<M: Mod> std::str::FromStr for ModInt<M> {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(ModInt::new(s.parse()?))
        }
    }
    impl<M> std::fmt::Display for ModInt<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> AddAssign<T> for ModInt<M> {
        fn add_assign(&mut self, other: T) {
            *self = *self + other;
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> SubAssign<T> for ModInt<M> {
        fn sub_assign(&mut self, other: T) {
            *self = *self - other;
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> MulAssign<T> for ModInt<M> {
        fn mul_assign(&mut self, other: T) {
            *self = *self * other;
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> DivAssign<T> for ModInt<M> {
        fn div_assign(&mut self, other: T) {
            *self = *self / other;
        }
    }
    impl<M: Mod> std::iter::Sum for ModInt<M> {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(0.into(), |acc, x| acc + x)
        }
    }
    impl<'a, M: Mod> std::iter::Sum<&'a ModInt<M>> for ModInt<M> {
        fn sum<I: Iterator<Item = &'a ModInt<M>>>(iter: I) -> Self {
            iter.fold(0.into(), |acc, &x| acc + x)
        }
    }
    impl<M: Mod> std::iter::Product for ModInt<M> {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(1.into(), |acc, x| acc * x)
        }
    }
    impl<'a, M: Mod> std::iter::Product<&'a ModInt<M>> for ModInt<M> {
        fn product<I: Iterator<Item = &'a ModInt<M>>>(iter: I) -> Self {
            iter.fold(1.into(), |acc, &x| acc * x)
        }
    }
    impl<M> std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ModInt({})", self.x)
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Mod1e9 {}
    impl Mod for Mod1e9 {
        const MOD: i64 = 1_000_000_007;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Mod998 {}
    impl Mod for Mod998 {
        const MOD: i64 = 998_244_353;
    }
}
pub type MInt = modint::ModInt<modint::Mod998>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    length: i64,
    sum_a: MInt,
    sum_b: MInt,
    sum_p: MInt,
}

impl Node {
    fn new(a: MInt, b: MInt) -> Self {
        Self {
            length: 1,
            sum_a: a,
            sum_b: b,
            sum_p: a * b,
        }
    }
}

impl Default for Node {
    fn default() -> Self {
        Self {
            length: 0,
            sum_a: 0.into(),
            sum_b: 0.into(),
            sum_p: 0.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Op {
    add_a: MInt,
    add_b: MInt,
}

impl Op {
    fn a(x: i64) -> Self {
        Self {
            add_a: x.into(),
            add_b: 0.into(),
        }
    }
    fn b(x: i64) -> Self {
        Self {
            add_a: 0.into(),
            add_b: x.into(),
        }
    }
}

impl Default for Op {
    fn default() -> Self {
        Self {
            add_a: 0.into(),
            add_b: 0.into(),
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        aa: [i64; n],
        bb: [i64; n],
    }
    let vec = aa
        .into_iter()
        .zip(bb.into_iter())
        .map(|(a, b)| Node::new(a.into(), b.into()))
        .collect_vec();
    let op = |l: &Node, r: &Node| Node {
        length: l.length + r.length,
        sum_a: l.sum_a + r.sum_a,
        sum_b: l.sum_b + r.sum_b,
        sum_p: l.sum_p + r.sum_p,
    };
    let mapping = |n: &Node, o: &Op| Node {
        length: n.length,
        sum_a: n.sum_a + o.add_a * n.length,
        sum_b: n.sum_b + o.add_b * n.length,
        sum_p: n.sum_p + o.add_b * n.sum_a + o.add_a * n.sum_b + o.add_a * o.add_b * n.length,
    };
    let composition = |o: &Op, p: &Op| Op {
        add_a: o.add_a + p.add_a,
        add_b: o.add_b + p.add_b,
    };
    let mut tree = LazySegtree::from_vec(
        vec,
        op,
        Node::default(),
        mapping,
        composition,
        Op::default(),
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
                tree.update(l, r, &Op::a(x));
            }
            2 => {
                input! {
                    l: Usize1,
                    r: usize,
                    x: i64,
                }
                tree.update(l, r, &Op::b(x));
            }
            3 => {
                input! {
                    l: Usize1,
                    r: usize,
                }
                println!("{}", tree.accum(l, r).sum_p);
            }
            _ => unreachable!(),
        }
    }
}

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

pub type MInt = modint::ModInt<modint::Mod998>;
pub mod modint {
    use std::ops::*;
    pub trait Mod: Copy {
        const MOD: i64;
    }
    #[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
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
        pub fn inv_fact(fact: &Vec<Self>) -> Vec<Self> {
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
            fact: &'a Vec<Self>,
            inv_fact: &'a Vec<Self>,
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
        fn add(self, other: T) -> Self {
            let other = other.into();
            let sum = self.x + other.x;
            Self::new_internal(if sum >= M::MOD { sum - M::MOD } else { sum })
        }
    }
    impl<M: Mod, T: Into<ModInt<M>>> Sub<T> for ModInt<M> {
        type Output = Self;
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
    impl<M: Mod> std::iter::Product for ModInt<M> {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(1.into(), |acc, x| acc * x)
        }
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Mod1e9 {}
    impl Mod for Mod1e9 {
        const MOD: i64 = 1_000_000_009;
    }
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum Mod998 {}
    impl Mod for Mod998 {
        const MOD: i64 = 998_244_353;
    }
}
pub mod fft {
    use super::modint::{self, Mod};
    use num_traits::{Float, FloatConst, One, Zero};
    use std::ops::*;
    pub trait DftRing: Copy + Add<Output = Self> + Sub<Output = Self> + Mul<Output = Self> {
        fn root(dim: usize) -> Self;
        fn zero() -> Self;
        fn one() -> Self;
        fn scale_inv(dim: usize) -> Self;
        fn inv(self) -> Self;
    }
    impl<T: Float + FloatConst> DftRing for num::complex::Complex<T> {
        fn root(dim: usize) -> Self {
            Self::from_polar(&T::one(), &(T::PI() / T::from(dim / 2).unwrap()))
        }
        fn zero() -> Self {
            Zero::zero()
        }
        fn one() -> Self {
            One::one()
        }
        fn scale_inv(dim: usize) -> Self {
            Self::from(T::from(dim).unwrap()).inv()
        }
        fn inv(self) -> Self {
            Self::inv(&self)
        }
    }
    impl DftRing for modint::ModInt<modint::Mod998> {
        fn root(dim: usize) -> Self {
            Self::new(3).pow((modint::Mod998::MOD - 1) / dim as i64)
        }
        fn zero() -> Self {
            0.into()
        }
        fn one() -> Self {
            1.into()
        }
        fn scale_inv(dim: usize) -> Self {
            Self::new(dim as i64).inv()
        }
        fn inv(self) -> Self {
            self.inv()
        }
    }
    pub fn fft<R: DftRing>(f: &mut [R], inverse: bool) {
        let n = f.len();
        assert!(n.is_power_of_two());
        {
            let mut i = 0;
            for j in 1..n - 1 {
                let mut k = n >> 1;
                loop {
                    i ^= k;
                    if k <= i {
                        break;
                    }
                    k >>= 1;
                }
                if j < i {
                    f.swap(i, j);
                }
            }
        }
        let mut roots = Vec::new();
        {
            let mut m = 1;
            let mut cur = if inverse {
                R::root(n).inv()
            } else {
                R::root(n)
            };
            while m < n {
                roots.push(cur);
                cur = cur * cur;
                m *= 2;
            }
        }
        let mut m = 1;
        while m < n {
            let base = roots.pop().unwrap();
            let mut r = 0;
            while r < n {
                let mut w = R::one();
                for s in r..r + m {
                    let u = f[s];
                    let d = f[s + m] * w;
                    f[s] = u + d;
                    f[s + m] = u - d;
                    w = w * base;
                }
                r += 2 * m;
            }
            m *= 2;
        }
    }
    pub fn convolution<R: DftRing>(f: &[R], g: &[R]) -> Vec<R> {
        use std::iter::repeat;
        let result_len = f.len() + g.len() - 1;
        let p = result_len.next_power_of_two();
        let mut f: Vec<_> = f.iter().cloned().chain(repeat(R::zero())).take(p).collect();
        let mut g: Vec<_> = g.iter().cloned().chain(repeat(R::zero())).take(p).collect();
        fft(&mut f, false);
        fft(&mut g, false);
        let inv = R::scale_inv(p);
        for i in 0..p {
            f[i] = f[i] * g[i] * inv;
        }
        fft(&mut f, true);
        f[..result_len].to_vec()
    }
}

#[fastout]
fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        k: usize,
        x: usize,
        y: usize,
        z: usize,
    }
    let (fact, inv_fact) = MInt::facts(*[r, g, b].iter().max().unwrap());
    let comb = MInt::comb(&fact, &inv_fact);
    let pattern = |lb: usize, ub: usize, i: usize| {
        if lb <= i && i <= ub {
            comb(ub, i)
        } else {
            0.into()
        }
    };
    let v_r = (0..=k).map(|i| pattern(k - y, r, i)).collect_vec();
    let v_g = (0..=k).map(|i| pattern(k - z, g, i)).collect_vec();
    let v_rg = fft::convolution(&v_r, &v_g);
    let mut ans = MInt::new(0);
    for i in 0..=k {
        ans += v_rg[i] * pattern(k - x, b, k - i);
    }

    println!("{}", ans);
}

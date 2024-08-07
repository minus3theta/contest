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

fn is_k_palindrome(x: usize, k: usize) -> bool {
    (0..k / 2).all(|i| (x >> i) & 1 == (x >> (k - i - 1)) & 1)
}

fn matching(s: &[char], p: usize, x: usize, k: usize) -> bool {
    (0..k).all(|i| matches!((s[p + i], (x >> i) & 1), ('?', _) | ('A', 0) | ('B', 1)))
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }
    let mut dp = vec![vec![MInt::new(0); 1 << k]; n - k + 1];
    for x in 0..1 << k {
        dp[0][x] = if matching(&s, 0, x, k) && !is_k_palindrome(x, k) {
            1.into()
        } else {
            0.into()
        };
    }
    for i in 0..n - k {
        for x in 0..1 << k {
            if matching(&s, i + 1, x, k) && !is_k_palindrome(x, k) {
                let y = (x << 1) & ((1 << k) - 1);
                let temp = dp[i][y] + dp[i][y + 1];
                dp[i + 1][x] += temp;
            }
        }
    }

    println!("{}", dp[n - k].iter().sum::<MInt>());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_k_palindrome() {
        assert!(is_k_palindrome(0b1001, 4));
    }

    #[test]
    fn test_matching() {
        assert!(matching(&['A', 'B', 'B', 'A'], 0, 0b0110, 4));
    }
}

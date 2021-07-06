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

use modint::*;
pub mod modint {
    const MOD: i64 = 1_000_000_007;
    #[derive(Copy, Clone, Hash, Debug, PartialEq, Eq)]
    pub struct ModInt {
        x: i64,
    }
    impl ModInt {
        pub fn new(x: i64) -> Self {
            let x = x % MOD;
            ModInt {
                x: if x < 0 { x + MOD } else { x },
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
            self.pow(MOD - 2)
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
    }
    impl<T: Into<ModInt>> std::ops::Add<T> for ModInt {
        type Output = Self;
        fn add(self, other: T) -> Self {
            let other = other.into();
            let sum = self.x + other.x;
            ModInt {
                x: if sum >= MOD { sum - MOD } else { sum },
            }
        }
    }
    impl<T: Into<ModInt>> std::ops::Sub<T> for ModInt {
        type Output = Self;
        fn sub(self, other: T) -> Self {
            let other = other.into();
            let sum = self.x - other.x;
            ModInt {
                x: if sum < 0 { sum + MOD } else { sum },
            }
        }
    }
    impl<T: Into<ModInt>> std::ops::Mul<T> for ModInt {
        type Output = Self;
        fn mul(self, other: T) -> Self {
            let other = other.into();
            ModInt {
                x: self.x * other.x % MOD,
            }
        }
    }
    impl<T: Into<ModInt>> std::ops::Div<T> for ModInt {
        type Output = Self;
        fn div(self, other: T) -> Self {
            let other = other.into();
            self * other.inv()
        }
    }
    impl From<i64> for ModInt {
        fn from(x: i64) -> Self {
            ModInt::new(x)
        }
    }
    impl std::str::FromStr for ModInt {
        type Err = <i64 as std::str::FromStr>::Err;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(ModInt::new(s.parse()?))
        }
    }
    impl std::fmt::Display for ModInt {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            self.x.fmt(f)
        }
    }
    impl<T: Into<ModInt>> std::ops::AddAssign<T> for ModInt {
        fn add_assign(&mut self, other: T) {
            *self = *self + other;
        }
    }
    impl<T: Into<ModInt>> std::ops::SubAssign<T> for ModInt {
        fn sub_assign(&mut self, other: T) {
            *self = *self - other;
        }
    }
    impl<T: Into<ModInt>> std::ops::MulAssign<T> for ModInt {
        fn mul_assign(&mut self, other: T) {
            *self = *self * other;
        }
    }
    impl<T: Into<ModInt>> std::ops::DivAssign<T> for ModInt {
        fn div_assign(&mut self, other: T) {
            *self = *self / other;
        }
    }
    impl std::iter::Sum for ModInt {
        fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(0.into(), |acc, x| acc + x)
        }
    }
    impl std::iter::Product for ModInt {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(1.into(), |acc, x| acc * x)
        }
    }
}

const BASE: i64 = 3;

fn convert(s: &[char], dict: (u8, u8, u8)) -> Vec<u8> {
    s.iter()
        .map(|&c| match c {
            'R' => dict.0,
            'G' => dict.1,
            'B' => dict.2,
            _ => unreachable!(),
        })
        .collect()
}

fn hash_array(s: &[u8]) -> Vec<ModInt> {
    let mut ret = vec![0.into(); s.len() + 1];
    for (i, &v) in s.iter().enumerate() {
        ret[i + 1] = ret[i] * BASE + v as i64;
    }
    ret
}

/// hash of s[l..r]
fn hash(h: &[ModInt], pow: &[ModInt], l: usize, r: usize) -> ModInt {
    h[r] - h[l] * pow[r - l]
}

fn get_pow(n: usize) -> Vec<ModInt> {
    let mut pow = vec![ModInt::new(1); n + 1];
    for i in 0..n {
        pow[i + 1] = pow[i] * BASE;
    }
    pow
}

fn solve(s: &[ModInt], t: &[ModInt], pow: &[ModInt]) -> i64 {
    let n = s.len() - 1;
    let mut ret = 0;
    for k in 1..=n {
        if hash(s, pow, 0, k) == hash(t, pow, n - k, n) {
            ret += 1;
        }
    }
    for k in 1..=n - 1 {
        if hash(s, pow, n - k, n) == hash(t, pow, 0, k) {
            ret += 1;
        }
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    let s = hash_array(&convert(&s, (0, 1, 2)));
    let pow = get_pow(n);
    let t_r = hash_array(&convert(&t, (0, 2, 1)));
    let t_g = hash_array(&convert(&t, (2, 1, 0)));
    let t_b = hash_array(&convert(&t, (1, 0, 2)));

    println!(
        "{}",
        solve(&s, &t_r, &pow) + solve(&s, &t_g, &pow) + solve(&s, &t_b, &pow)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash1() {
        let s = hash_array(&[0, 1, 2, 0]);
        let t = hash_array(&[1, 2, 0]);
        let pow = get_pow(4);
        assert_eq!(hash(&s, &pow, 1, 4), hash(&t, &pow, 0, 3));
        assert_ne!(hash(&s, &pow, 0, 3), hash(&t, &pow, 0, 3));
    }
}

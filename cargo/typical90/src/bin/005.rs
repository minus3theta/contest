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

const MOD: i64 = 1_000_000_007;

#[derive(Copy, Clone, Hash, Debug)]
struct ModInt {
    x: i64,
}
impl ModInt {
    fn new(x: i64) -> Self {
        let x = x % MOD;
        ModInt {
            x: if x < 0 { x + MOD } else { x },
        }
    }
    fn pow(self, e: i64) -> Self {
        if e == 0 {
            Self::new(1)
        } else if e % 2 == 1 {
            self.pow(e - 1) * self
        } else {
            let y = self.pow(e / 2);
            y * y
        }
    }
    #[allow(dead_code)]
    fn inv(self) -> Self {
        self.pow(MOD - 2)
    }
    #[allow(dead_code)]
    fn fact(n: usize) -> Vec<Self> {
        let mut ret = vec![0.into(); n + 1];
        ret[0] = 1.into();
        for i in 1..n + 1 {
            ret[i] = ret[i - 1] * i as i64;
        }
        ret
    }
    #[allow(dead_code)]
    fn inv_fact(fact: &Vec<Self>) -> Vec<Self> {
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

fn pow_10(x: i64, b: usize) -> usize {
    if x == 0 {
        1
    } else if x % 2 == 1 {
        pow_10(x - 1, b) * 10 % b
    } else {
        let p = pow_10(x / 2, b);
        p * p % b
    }
}

fn solve(n: i64, b: usize, cs: &[usize]) -> Vec<ModInt> {
    let mut ret = vec![ModInt::new(0); b];
    match n {
        0 => {
            ret[0] = 1.into();
        }
        _ if n % 2 == 1 => {
            let prev = solve(n - 1, b, cs);
            for (i, &m) in prev.iter().enumerate() {
                for &c in cs {
                    ret[(i * 10 + c) % b] += m;
                }
            }
        }
        _ => {
            let prev = solve(n / 2, b, cs);
            let pow = pow_10(n / 2, b);
            for (i, &m) in prev.iter().enumerate() {
                for (j, &p) in prev.iter().enumerate() {
                    ret[(i * pow + j) % b] += m * p;
                }
            }
        }
    }

    ret
}

#[fastout]
fn main() {
    input! {
        n: i64,
        b: usize,
        k: usize,
        cs: [usize; k],
    }

    println!("{}", solve(n, b, &cs)[0]);
}

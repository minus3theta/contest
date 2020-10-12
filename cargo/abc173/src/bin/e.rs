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

fn optimal(pos: &mut Vec<i64>, neg: &mut Vec<i64>, mut k: usize) -> Option<ModInt> {
    let mut ans = ModInt::new(1);
    if k % 2 == 1 {
        k -= 1;
        match pos.pop() {
            Some(x) => ans *= x,
            None => return None,
        }
    }
    while k > 0 {
        k -= 2;
        let pl = pos.len();
        let p = if pl >= 2 {
            Some(pos[pl - 1] * pos[pl - 2])
        } else {
            None
        };
        let nl = neg.len();
        let n = if nl >= 2 {
            Some(neg[nl - 1] * neg[nl - 2])
        } else {
            None
        };
        match (p, n) {
            (None, None) => return None,
            (Some(x), None) => {
                ans *= x;
                pos.pop();
                pos.pop();
            }
            (Some(x), Some(y)) if x >= y => {
                ans *= x;
                pos.pop();
                pos.pop();
            }
            (_, Some(y)) => {
                ans *= y;
                neg.pop();
                neg.pop();
            }
        }
    }
    Some(ans)
}

fn solve(pos: Vec<i64>, neg: Vec<i64>, k: usize, has_zero: bool) -> ModInt {
    if let Some(x) = optimal(&mut pos.clone(), &mut neg.clone(), k) {
        return x;
    }
    if has_zero {
        return 0.into();
    }
    let mut ans = ModInt::new(1);
    for &x in pos.iter().chain(neg.iter()).take(k) {
        ans *= x;
    }
    ans
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [i64; n],
    }
    let mut pos: Vec<_> = aa.iter().filter(|&&a| a > 0).cloned().collect();
    let mut neg: Vec<_> = aa.iter().filter(|&&a| a < 0).cloned().collect();
    pos.sort();
    neg.sort_by_key(|&a| cmp::Reverse(a));
    let has_zero = pos.len() + neg.len() < n;

    println!("{}", solve(pos, neg, k, has_zero));
}

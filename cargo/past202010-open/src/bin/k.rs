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

const MOD: i64 = 1_000_000_000;

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

// 1-indexed
// example: Bit::new(n, |x, y| *x += *y, 0)
struct Bit<T, F> {
    n: usize,
    dat: Vec<T>,
    op: F,
    unit: T,
}

impl<T: Clone, F: Fn(&mut T, &T)> Bit<T, F> {
    #[allow(dead_code)]
    fn new(n: usize, op: F, unit: T) -> Self {
        Bit {
            n: n,
            dat: vec![unit.clone(); n + 1],
            op: op,
            unit: unit,
        }
    }
    #[allow(dead_code)]
    fn from_vec(mut v: Vec<T>, op: F, unit: T) -> Self {
        let n = v.len();
        let mut dat = vec![unit.clone()];
        dat.append(&mut v);
        for i in 1..n {
            let j = i as i32;
            let b = (j & -j) as usize;
            let x = dat[i].clone();
            op(&mut dat[i + b], &x);
        }
        Bit {
            n: n,
            dat: dat,
            op: op,
            unit: unit,
        }
    }
    fn operate(&mut self, k: usize, a: &T) {
        let mut k = k;
        while k <= self.n {
            (self.op)(&mut self.dat[k], &a);
            let l = k as i32;
            k += (l & -l) as usize;
        }
    }
    fn accum(&self, k: usize) -> T {
        let mut k = k;
        let mut sum = self.unit.clone();
        while k > 0 {
            (self.op)(&mut sum, &self.dat[k]);
            let l = k as i32;
            k -= (l & -l) as usize;
        }
        sum
    }
}

#[fastout]
fn main() {
    input! {
        k: usize,
        aa: [[Usize1]; k],
        q: usize,
        bb: [Usize1; q],
    }
    let internal: Vec<_> = aa.iter().map(inversion).collect();
    // dbg!(&internal);
    let popls: Vec<_> = aa.iter().map(count_popl).collect();
    let mut ans = ModInt::new(0);
    let mut popl = vec![0; 20];
    for &b in &bb {
        ans += internal[b];
        for i in 0..20 {
            for j in 0..i {
                ans += popl[i] * popls[b][j];
            }
        }
        for i in 0..20 {
            popl[i] += popls[b][i];
        }
    }

    println!("{}", ans);
}

fn inversion(a: &Vec<usize>) -> i64 {
    let mut bit = Bit::new(21, |x, y| *x += y, 0i64);
    let mut inv = 0;
    for &x in a.iter().rev() {
        inv += bit.accum(x + 1);
        bit.operate(x + 2, &1);
    }
    inv
}

fn count_popl(a: &Vec<usize>) -> Vec<i64> {
    let mut popl = vec![0; 20];
    for &x in a {
        popl[x] += 1;
    }
    popl
}

#[test]
fn test_inversion() {
    assert_eq!(inversion(&vec![1, 1]), 0);
    assert_eq!(inversion(&vec![1, 20]), 0);
    assert_eq!(inversion(&vec![20, 1]), 1);
}

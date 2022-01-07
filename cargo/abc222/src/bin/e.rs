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
    impl<M: Mod> std::iter::Product for ModInt<M> {
        fn product<I: Iterator<Item = Self>>(iter: I) -> Self {
            iter.fold(1.into(), |acc, x| acc * x)
        }
    }
    impl<M> std::fmt::Debug for ModInt<M> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ModInt ({})", self.x)
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

fn route(adj: &[Vec<(usize, usize)>], a: usize, b: usize, count: &mut [usize]) {
    let n = adj.len();
    let mut prev = vec![None; n];
    dfs(adj, a, a, &mut prev);
    let mut current = b;
    while let Some((i, v)) = prev[current] {
        count[i] += 1;
        current = v;
    }
}

fn dfs(adj: &[Vec<(usize, usize)>], start: usize, v: usize, prev: &mut [Option<(usize, usize)>]) {
    for &(i, u) in &adj[v] {
        if u == start || prev[u].is_some() {
            continue;
        }
        prev[u] = Some((i, v));
        dfs(adj, start, u, prev);
    }
}

fn pattern(count: &[usize], total: usize, k: isize) -> MInt {
    let r_2 = total as isize + k;
    if r_2 % 2 != 0 || r_2 < 0 {
        return 0.into();
    }
    let r = (r_2 / 2) as usize;
    if r > total {
        return 0.into();
    }
    let mut dp = vec![vec![MInt::new(0); total + 1]; count.len() + 1];
    dp[0][0] = 1.into();
    for (i, &c) in count.iter().enumerate() {
        for p in 0..=total {
            dp[i + 1][p] = dp[i][p];
            if let Some(q) = p.checked_sub(c) {
                let v = dp[i][q];
                dp[i + 1][p] += v;
            }
        }
    }
    dp[count.len()][r]
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        k: isize,
        a: [Usize1; m],
        es: [(Usize1, Usize1); n-1],
    }
    let mut adj = vec![vec![]; n];
    for (i, (u, v)) in es.into_iter().enumerate() {
        adj[u].push((i, v));
        adj[v].push((i, u));
    }
    let mut count = vec![0; n - 1];
    for (&a, &b) in a.iter().tuple_windows() {
        route(&adj, a, b, &mut count);
    }
    let total = count.iter().sum::<usize>();

    println!("{}", pattern(&count, total, k));
}

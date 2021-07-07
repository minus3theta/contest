use cargo_snippet::snippet;

pub use crate::modint::modint;

#[snippet]
#[snippet(include = "modint")]
#[snippet(prefix = "pub type MInt = modint::ModInt<modint::Mod998>;")]
pub mod fft {
    use std::ops::*;

    use num_traits::{Float, FloatConst, One, Zero};

    use super::modint::{self, Mod};

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

#[cfg(test)]
mod tests {
    use num::complex::Complex64;

    use super::*;
    use crate::{
        fft::fft::convolution,
        modint::modint::{Mod998, ModInt},
    };
    use fft::DftRing;

    #[test]
    fn test_root_f64() {
        let r = Complex64::root(8);
        assert!((r.powu(8) - Complex64::new(1.0, 0.0)).norm() < 1e-10);
        assert!((r.powu(4) - Complex64::new(1.0, 0.0)).norm() > 1e-10);
    }

    #[test]
    fn test_root_mod() {
        let r = ModInt::<Mod998>::root(8);
        assert_eq!(r.pow(8), 1.into());
        assert_ne!(r.pow(4), 1.into());
    }

    #[test]
    fn test_convolution_mod() {
        let mut a: Vec<ModInt<Mod998>> = vec![8.into(), 6.into(), 9.into()];
        let mut b = vec![1.into(), 2.into(), 0.into()];
        assert_eq!(
            convolution(&mut a, &mut b),
            vec![8.into(), 22.into(), 21.into(), 18.into(), 0.into()]
        );
    }
}

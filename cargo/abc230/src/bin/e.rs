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

fn solve(n: i64) -> i64 {
    let mut sum = 0;
    for i in 1.. {
        if i * i > n {
            break;
        }
        sum += n / i;
        if i != n / i {
            sum += (n / i - n / (i + 1)) * i;
        }
    }
    sum
}

#[fastout]
fn main() {
    input! {
        n: i64,
    }

    println!("{}", solve(n));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive(n: i64) -> i64 {
        (1..=n).map(|i| n / i).sum()
    }

    #[test]
    fn compare_naive() {
        for n in 1..=1000 {
            assert_eq!(solve(n), naive(n), "{}", n);
        }
    }

    #[test]
    fn test_5() {
        assert_eq!(solve(5), naive(5));
    }
}

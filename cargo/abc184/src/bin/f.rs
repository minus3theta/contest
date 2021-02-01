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

fn patterns(a: &[i64]) -> Vec<i64> {
    let l = a.len();
    let mut ret = vec![];
    for p in 0..(1 << l) {
        let mut sum = 0;
        for (i, &x) in a.iter().enumerate() {
            if (1 << i) & p != 0 {
                sum += x;
            }
        }
        ret.push(sum);
    }
    ret.sort();
    ret.dedup();
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        t: i64,
        a: [i64; n],
    }
    let (fst, snd) = a.split_at(n / 2);
    let fst = patterns(fst);
    let snd = patterns(snd);
    let mut ans = 0;
    for &x in &fst {
        if x > t {
            break;
        }
        let time = match snd.binary_search(&(t - x)) {
            Ok(_) => t,
            Err(i) => {
                let i = i.saturating_sub(1);
                x + snd[i]
            }
        };
        ans = cmp::max(ans, time);
    }

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pat_0() {
        assert_eq!(patterns(&[]), vec![0]);
    }
}

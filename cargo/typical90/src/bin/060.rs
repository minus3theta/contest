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

fn lower_bound<T: std::cmp::PartialOrd>(arr: &[T], value: &T) -> usize {
    let mut l = -1;
    let mut r = arr.len() as isize;
    while l + 1 != r {
        let m = (l + r) / 2;
        if &arr[m as usize] < value {
            l = m;
        } else {
            r = m;
        }
    }
    r as usize
}

fn lis<T: std::cmp::PartialOrd + Clone>(arr: &[T]) -> Vec<usize> {
    use std::cmp::Reverse;

    let mut ret = vec![0; arr.len()];
    let mut minimum = vec![Reverse(None); arr.len()];
    for (i, x) in arr.iter().enumerate() {
        let pos = lower_bound(&minimum, &Reverse(Some(Reverse(x.clone()))));
        minimum[pos] = Reverse(Some(Reverse(x.clone())));
        ret[i] = pos + 1;
    }
    ret
}

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
    }
    let lis_len = lis(&a);
    a.reverse();
    let mut lds_len = lis(&a);
    lds_len.reverse();
    let ans = lis_len
        .into_iter()
        .zip(lds_len)
        .map(|(x, y)| x + y - 1)
        .max()
        .unwrap();

    println!("{}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;
    const V: &[i32] = &[3, 1, 4, 6, 5];
    #[test]
    fn test_lower_bound() {
        assert_eq!(lower_bound(V, &4), 2);
        assert_eq!(lower_bound(V, &0), 0);
        assert_eq!(lower_bound(V, &10), 5);
    }
    #[test]
    fn test_lis() {
        assert_eq!(lis(&[4, 2, 3, 1, 5]), vec![1, 1, 2, 1, 3]);
    }
}

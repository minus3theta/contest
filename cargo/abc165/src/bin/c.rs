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

fn solve(n: usize, m: usize, arr: &mut Vec<usize>, req: &Vec<(usize, usize, usize, i64)>) -> i64 {
  if arr.len() == n {
    req
      .iter()
      .filter(|(a, b, c, _)| arr[*a] + c == arr[*b])
      .map(|&(_, _, _, d)| d)
      .sum()
  } else {
    let mut ret = 0;
    for tail in *arr.last().unwrap()..=m {
      arr.push(tail);
      ret = cmp::max(ret, solve(n, m, arr, req));
      arr.pop();
    }
    ret
  }
}

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    q: usize,
    req: [(Usize1, Usize1, usize, i64); q],
  }

  println!("{}", solve(n, m, &mut vec![1], &req));
}

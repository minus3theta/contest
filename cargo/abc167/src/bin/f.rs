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

fn solve(ss: &Vec<(i32, i32)>) -> bool {
  let sum: i32 = ss.iter().map(|&(_, l)| l).sum();
  if sum != 0 {
    return false;
  }
  let mut gain: i32 = ss
    .iter()
    .filter_map(|&(b, l)| if b >= 0 { Some(l) } else { None })
    .sum();
  let (mut up, mut down): (Vec<(i32, i32)>, Vec<(i32, i32)>) = ss
    .iter()
    .filter(|&&(b, _)| b < 0)
    .partition(|&&(_, l)| l >= 0);
  up.sort_by_key(|&(b, _)| -b);
  down.sort_by_key(|&(b, l)| b - l);
  for &(b, l) in &up {
    if gain + b < 0 {
      return false;
    }
    gain += l;
  }
  for &(b, l) in &down {
    if gain + b < 0 {
      return false;
    }
    gain += l;
  }
  gain == 0
}

#[fastout]
fn main() {
  input! {
    n: usize,
    ss: [Chars; n],
  }
  let ss: Vec<_> = ss
    .into_iter()
    .map(|s| {
      let mut current = 0;
      let mut bottom = 0;
      for c in s.into_iter() {
        if c == '(' {
          current += 1;
        } else {
          current -= 1;
        }
        bottom = cmp::min(bottom, current);
      }
      (bottom, current)
    })
    .collect();

  println!("{}", if solve(&ss) { "Yes" } else { "No" });
}

#[allow(unused_imports)]
use std::cmp;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::fastout;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
  input! {
    n: usize,
    q: usize,
    aa: [i64; n],
    ss: [i64; q],
  }
  let mut acc = vec![(0, 0)];
  let mut g = 0;
  for (i, &a) in aa.iter().enumerate() {
    let h = a.gcd(&g);
    if h != g {
      acc.push((h, i+1));
    }
    g = h;
  }
  for &s in &ss {
    let h = g.gcd(&s);
    if h == 1 {
      for &(h, i) in &acc {
        if h.gcd(&s) == 1 {
          println!("{}", i);
          break;
        }
      }
    } else {
      println!("{}", h);
    }
  }
}

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

fn solve(x: i64) -> (i64, i64) {
  for a in 0i64.. {
    let pa = a.pow(5);
    if pa > x {
      break;
    }
    for b in 0i64.. {
      let pb = b.pow(5);
      if pb > x {
        break;
      }
      if pa + pb == x {
        return (a, -b);
      }
    }
  }
  for b in 0i64.. {
    let pb = b.pow(5);
    for k in 0i64.. {
      let pk = k.pow(5);
      if pk > x {
        break;
      }
      if (b + k).pow(5) - pb == x {
        return (b + k, b);
      }
    }
  }
  todo!()
}

#[fastout]
fn main() {
  input! {
    x: i64,
  }
  let (a, b) = solve(x);

  println!("{} {}", a, b);
}

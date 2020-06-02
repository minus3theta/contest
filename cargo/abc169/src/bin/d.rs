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

#[fastout]
fn main() {
  input! {
    mut n: i64,
  }
  use std::collections::BTreeMap;
  let mut factor = BTreeMap::new();
  for i in 2.. {
    if i * i > n {
      break;
    }
    while n % i == 0 {
      n /= i;
      *factor.entry(i).or_insert(0i64) += 1;
    }
  }
  if n != 1 {
    *factor.entry(n).or_insert(0) += 1;
  }
  let mut ans = 0;
  for &e in factor.values() {
    let mut e = e;
    for i in 1.. {
      if i > e {
        break;
      }
      e -= i;
      ans += 1;
    }
  }

  println!("{}", ans);
}

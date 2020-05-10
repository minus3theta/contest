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
    n: usize,
    m: usize,
    x: i64,
    book: [(i64, [i64; m]); n],
  }
  let mut ans = 1 << 60;
  for pat in 0..1 << n {
    let mut und = vec![0; m];
    let mut price = 0;
    for b in 0..n {
      if (pat >> b) & 1 != 0 {
        price += book[b].0;
        for algo in 0..m {
          und[algo] += book[b].1[algo];
        }
      }
    }
    if und.iter().all(|&u| u >= x) {
      ans = cmp::min(price, ans)
    }
  }
  if ans == 1 << 60 {
    ans = -1;
  }

  println!("{}", ans);
}

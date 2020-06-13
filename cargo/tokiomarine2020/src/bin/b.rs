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
      a: i64,
      v: i64,
      b: i64,
      w: i64,
      t: i64,
    }
    let d = (a - b).abs();
    let x = v - w;

    println!("{}", if t * x >= d { "YES" } else { "NO" });
}

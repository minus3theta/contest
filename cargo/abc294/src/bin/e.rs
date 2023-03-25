use either::Either;
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

#[fastout]
fn main() {
    input! {
        _l: i64,
        n1: usize,
        n2: usize,
        a1: [(i32, i64); n1],
        a2: [(i32, i64); n2],
    }

    let mut ev = Vec::new();
    {
        let mut pos = 0;
        for &(v, l) in &a1 {
            ev.push((pos, Either::Left(v)));
            pos += l;
        }
        ev.push((pos, Either::Left(0)));
    }
    {
        let mut pos = 0;
        for &(v, l) in &a2 {
            ev.push((pos, Either::Right(v)));
            pos += l;
        }
        ev.push((pos, Either::Right(0)));
    }
    ev.sort();

    let mut ans = 0;
    let mut x1 = 0;
    let mut x2 = 0;
    let mut prev = 0;
    for &(pos, e) in &ev {
        if x1 == x2 {
            ans += pos - prev;
        }
        match e {
            Either::Left(x) => x1 = x,
            Either::Right(x) => x2 = x,
        }
        prev = pos;
    }

    println!("{}", ans);
}

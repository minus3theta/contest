#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;
use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    let n: i32 = buf.trim().parse().unwrap();
    let mut available: BTreeSet<_> = (1..=2 * n + 1).collect();
    while let Some(x) = available.pop_first() {
        println!("{}", x);
        buf.clear();
        stdin.read_line(&mut buf).unwrap();
        let a: i32 = buf.trim().parse().unwrap();
        available.remove(&a);
    }
}

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
        n: usize,
        k: usize,
        ss: [String; n],
    }
    let mut popl = BTreeMap::new();
    for s in ss.into_iter() {
        *popl.entry(s).or_insert(0) += 1;
    }
    let mut array: Vec<(String, i32)> = popl.into_iter().collect();
    array.sort_by_key(|(_, p)| cmp::Reverse(*p));
    let ans_popl = array[k - 1].1;
    if array.iter().filter(|(_, p)| *p == ans_popl).count() > 1 {
        println!("AMBIGUOUS");
    } else {
        println!("{}", array[k - 1].0);
    }
}

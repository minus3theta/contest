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

#[derive(Debug, Default)]
struct Types {
    popl: BTreeMap<i64, usize>,
    kind: usize,
}

impl Types {
    pub fn insert(&mut self, value: i64) {
        if *self.popl.get(&value).unwrap_or(&0) == 0 {
            self.kind += 1;
        }
        *self.popl.entry(value).or_insert(0) += 1;
    }
    pub fn remove(&mut self, value: i64) {
        *self.popl.get_mut(&value).unwrap() -= 1;
        if *self.popl.get(&value).unwrap() == 0 {
            self.kind -= 1;
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        aa: [i64; n],
    }
    let mut types = Types::default();
    let mut head = 0;
    let mut ans = 0;
    for tail in 0..n {
        types.insert(aa[tail]);
        while types.kind > k {
            types.remove(aa[head]);
            head += 1;
        }
        ans = ans.max(tail + 1 - head);
    }

    println!("{}", ans);
}

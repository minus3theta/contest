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
use std::cmp::Reverse;
#[allow(unused_imports)]
use std::collections::*;

#[fastout]
fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        val: [(Usize1, Usize1, i64); n],
    }
    let mut map = BTreeMap::new();
    for (i, &(r, c, a)) in val.iter().enumerate() {
        map.entry(Reverse(a))
            .or_insert_with(|| vec![])
            .push((i, r, c));
    }
    let mut row_max = HashMap::new();
    let mut col_max = HashMap::new();
    let mut ans = vec![0; n];
    for v in map.values() {
        for &(i, r, c) in v {
            let mut ret = 0;
            if let Some(&ar) = row_max.get(&r) {
                ret = ret.max(ar);
            }
            if let Some(&ac) = col_max.get(&c) {
                ret = ret.max(ac);
            }
            ans[i] = ret;
        }
        for &(i, r, c) in v {
            let a = ans[i] + 1;
            row_max
                .entry(r)
                .and_modify(|m| *m = (*m).max(a))
                .or_insert(a);
            col_max
                .entry(c)
                .and_modify(|m| *m = (*m).max(a))
                .or_insert(a);
        }
    }
    for a in ans {
        println!("{}", a);
    }
}

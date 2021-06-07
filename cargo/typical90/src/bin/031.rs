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

fn grundy(w: i32, b: i32, memo: &mut BTreeMap<(i32, i32), i32>) -> i32 {
    if let Some(&g) = memo.get(&(w, b)) {
        return g;
    }
    let g = if w == 0 && b <= 1 {
        0
    } else {
        let mut targ = BTreeSet::new();
        if w >= 1 {
            targ.insert(grundy(w - 1, b + w, memo));
        }
        for k in 1..=b / 2 {
            targ.insert(grundy(w, b - k, memo));
        }
        (0..).filter(|g| !targ.contains(g)).next().unwrap()
    };
    memo.insert((w, b), g);
    g
}

#[fastout]
fn main() {
    input! {
        n: usize,
        ws: [i32; n],
        bs: [i32; n],
    }
    let mut memo = BTreeMap::new();
    let mut g = 0;
    for (w, b) in ws.into_iter().zip(bs) {
        g ^= grundy(w, b, &mut memo);
    }

    println!("{}", if g != 0 { "First" } else { "Second" });
}

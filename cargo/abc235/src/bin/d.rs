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

fn rotate(x: i64) -> Option<i64> {
    if x < 10 || x % 10 == 0 {
        None
    } else {
        let s = format!("{}{}", x % 10, x / 10);
        s.parse().ok()
    }
}

fn is_inside(x: i64, n: i64) -> bool {
    format!("{}", x).len() <= format!("{}", n).len()
}

#[fastout]
fn main() {
    input! {
        a: i64,
        n: i64,
    }
    let mut que = VecDeque::new();
    que.push_back(1);
    let mut dist = HashMap::new();
    dist.insert(1, 0);
    while let Some(x) = que.pop_front() {
        for y in Some(x * a).into_iter().chain(rotate(x)) {
            if !is_inside(y, n) || dist.contains_key(&y) {
                continue;
            }
            dist.insert(y, *dist.get(&x).unwrap() + 1);
            que.push_back(y);
        }
    }
    println!("{}", dist.get(&n).unwrap_or(&-1));
}

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
        n: usize,
        k: usize,
        p: [usize; n],
    }
    let mut que: BinaryHeap<_> = p.iter().take(k - 1).map(|&x| Reverse(x)).collect();
    que.push(Reverse(0));
    for &v in p.iter().skip(k - 1) {
        let &Reverse(top) = que.peek().unwrap();
        if v >= top {
            que.push(Reverse(v));
            que.pop();
        }
        println!("{}", que.peek().unwrap().0);
    }
}

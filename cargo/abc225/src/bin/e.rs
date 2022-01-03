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

#[derive(Debug, PartialEq, Eq)]
struct Point(i64, i64);

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.1 * other.0).cmp(&(other.1 * self.0))
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    }
    let ranges = xy
        .into_iter()
        .map(|(x, y)| (Point(x - 1, y), Point(x, y - 1)))
        .sorted()
        .collect_vec();
    let mut last = Point(1, 0);
    let mut ans = 0;
    for (end, begin) in ranges {
        if begin < last {
            continue;
        }
        ans += 1;
        last = end;
    }

    println!("{}", ans);
}

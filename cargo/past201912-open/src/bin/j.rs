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

fn dijkstra(si: usize, sj: usize, a: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let h = a.len();
    let w = a[0].len();
    let mut dist = vec![vec![i64::MAX; w]; h];
    dist[si][sj] = 0;
    let mut heap = std::collections::BinaryHeap::new();
    heap.push((0, si, sj));

    while let Some((d, i, j)) = heap.pop() {
        if d > dist[i][j] {
            continue;
        }
        for (ni, nj) in [
            (i.wrapping_sub(1), j),
            (i + 1, j),
            (i, j.wrapping_sub(1)),
            (i, j + 1),
        ] {
            if ni < h && nj < w {
                let nd = d + a[ni][nj];
                if nd < dist[ni][nj] {
                    dist[ni][nj] = nd;
                    heap.push((nd, ni, nj));
                }
            }
        }
    }
    dist
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[i64; w]; h],
    }
    let d0 = dijkstra(h - 1, 0, &a);
    let d1 = dijkstra(h - 1, w - 1, &a);
    let d2 = dijkstra(0, w - 1, &a);
    let mut ans = i64::MAX;
    for i in 0..h {
        for j in 0..w {
            let d = d0[i][j] + d1[i][j] + d2[i][j] - a[i][j] * 2;
            ans = ans.min(d);
        }
    }

    println!("{}", ans);
}

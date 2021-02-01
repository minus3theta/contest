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

const INF: i32 = 1 << 30;
const DIRS: &[(usize, usize)] = &[(0, 1), (1, 0), (0, std::usize::MAX), (std::usize::MAX, 0)];

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        start: (Usize1, Usize1),
        goal: (Usize1, Usize1),
        s: [Chars; h],
    }
    let mut que = VecDeque::new();
    que.push_back(start);
    let mut dist = vec![vec![INF; w]; h];
    dist[start.0][start.1] = 0;
    let is_open = |i: usize, j: usize| i < h && j < w && s[i][j] == '.';
    while let Some((i, j)) = que.pop_front() {
        let d = dist[i][j];
        for &(di, dj) in DIRS {
            let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
            if is_open(ni, nj) {
                if d < dist[ni][nj] {
                    dist[ni][nj] = d;
                    que.push_front((ni, nj));
                }
            }
        }
        for &di in &[std::usize::MAX - 1, std::usize::MAX, 0, 1, 2] {
            for &dj in &[std::usize::MAX - 1, std::usize::MAX, 0, 1, 2] {
                let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
                if is_open(ni, nj) {
                    if d + 1 < dist[ni][nj] {
                        dist[ni][nj] = d + 1;
                        que.push_back((ni, nj));
                    }
                }
            }
        }
    }
    let ans = dist[goal.0][goal.1];

    println!("{}", if ans == INF { -1 } else { ans });
}

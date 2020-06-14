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

#[fastout]
fn main() {
    let dx = [0, 1, 0, std::usize::MAX];
    let dy = [1, 0, std::usize::MAX, 0];
    input! {
        h: usize,
        w: usize,
        k: usize,
        src: (Usize1, Usize1),
        dst: (Usize1, Usize1),
        field: [Chars; h],
    }
    let mut dist = vec![vec![1i64 << 60; w]; h];
    dist[src.0][src.1] = 0;
    let mut que = std::collections::VecDeque::new();
    que.push_back(src);
    while let Some((x, y)) = que.pop_front() {
        let d = dist[x][y];
        for i in 0..4 {
            let mut cx = x;
            let mut cy = y;
            for _ in 0..k {
                let nx = cx.wrapping_add(dx[i]);
                let ny = cy.wrapping_add(dy[i]);
                if nx < h && ny < w && field[nx][ny] == '.' {
                    let nd = dist[nx][ny];
                    if nd <= d {
                        break;
                    }
                    if d + 1 < nd {
                        dist[nx][ny] = d + 1;
                        que.push_back((nx, ny));
                    }
                } else {
                    break;
                }
                cx = nx;
                cy = ny;
            }
        }
    }

    let d = dist[dst.0][dst.1];
    println!("{}", if d == (1i64 << 60) { -1 } else { d });
}

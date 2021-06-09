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

const INF: i64 = 1 << 60;
const DIRS: &[(usize, usize)] = &[(0, 1), (1, 0), (0, std::usize::MAX), (std::usize::MAX, 0)];

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        rs: Usize1,
        cs: Usize1,
        rt: Usize1,
        ct: Usize1,
        fld: [Chars; h],
    }
    let mut dist = vec![vec![vec![INF; 4]; w]; h];
    let mut que = VecDeque::new();
    for dir in 0..4 {
        dist[rs][cs][dir] = 0;
        que.push_back((rs, cs, dir));
    }
    while let Some((r, c, f)) = que.pop_front() {
        for dir in 0..4 {
            let (dr, dc) = DIRS[dir];
            let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
            if nr >= h || nc >= w || fld[nr][nc] == '#' {
                continue;
            }
            let d = dist[r][c][f] + if dir == f { 0 } else { 1 };
            if d < dist[nr][nc][dir] {
                dist[nr][nc][dir] = d;
                if dir == f {
                    que.push_front((nr, nc, dir));
                } else {
                    que.push_back((nr, nc, dir));
                }
            }
        }
    }

    println!("{}", dist[rt][ct].iter().min().unwrap());
}

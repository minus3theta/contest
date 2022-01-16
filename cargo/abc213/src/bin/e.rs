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

const NEG: usize = std::usize::MAX;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        fld: [Chars; h],
    }
    let mut que = VecDeque::new();
    que.push_back((0, 0));
    let mut dist = vec![vec![None; w]; h];
    dist[0][0] = Some(0);
    while let Some((r, c)) = que.pop_front() {
        let d = dist[r][c].unwrap();
        for &(dr, dc) in &[(0, 1), (1, 0), (0, NEG), (NEG, 0)] {
            let nr = r.wrapping_add(dr);
            let nc = c.wrapping_add(dc);
            if nr >= h || nc >= w {
                continue;
            }
            if let Some(nd) = dist[nr][nc] {
                if nd <= d {
                    continue;
                }
            }
            if fld[nr][nc] == '.' {
                dist[nr][nc] = Some(d);
                que.push_front((nr, nc));
            } else {
                dist[nr][nc] = Some(d + 1);
                que.push_back((nr, nc));
            }
        }
        for dr in -2isize..=2 {
            for dc in -2isize..=2 {
                let drc = dr.abs() + dc.abs();
                if drc <= 1 || drc == 4 {
                    continue;
                }
                let nr = (r as isize + dr) as usize;
                let nc = (c as isize + dc) as usize;
                if nr >= h || nc >= w || dist[nr][nc].is_some() {
                    continue;
                }
                dist[nr][nc] = Some(d + 1);
                que.push_back((nr, nc));
            }
        }
    }

    println!("{}", dist[h - 1][w - 1].unwrap());
}

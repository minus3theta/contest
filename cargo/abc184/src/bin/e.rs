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

const DIRS: &[(usize, usize)] = &[(0, 1), (1, 0), (0, std::usize::MAX), (std::usize::MAX, 0)];
const INF: i32 = 1 << 30;

fn bfs(fld: &Vec<Vec<char>>, s: (usize, usize)) -> Vec<Vec<i32>> {
    let h = fld.len();
    let w = fld[0].len();
    let mut dist = vec![vec![INF; w]; h];
    let mut used = vec![false; 26];
    let mut que = VecDeque::new();
    que.push_back(s);
    dist[s.0][s.1] = 0;
    while let Some((r, c)) = que.pop_front() {
        assert_ne!(fld[r][c], '#');
        let d = dist[r][c];
        let mut update = |nr: usize, nc: usize| {
            if d + 1 < dist[nr][nc] {
                dist[nr][nc] = d + 1;
                que.push_back((nr, nc));
            }
        };
        for &(dr, dc) in DIRS {
            let (nr, nc) = (r.wrapping_add(dr), c.wrapping_add(dc));
            if nr >= h || nc >= w || fld[nr][nc] == '#' {
                continue;
            }
            update(nr, nc);
        }
        if fld[r][c].is_ascii_lowercase() {
            let x = fld[r][c] as usize - 'a' as usize;
            if !used[x] {
                for (nr, row) in fld.iter().enumerate() {
                    for (nc, &nx) in row.iter().enumerate() {
                        if nx == fld[r][c] {
                            update(nr, nc);
                        }
                    }
                }
                used[x] = true;
            }
        }
    }
    dist
}

#[fastout]
fn main() {
    input! {
        h: usize,
        _w: usize,
        fld: [Chars; h],
    }
    let mut s = (0, 0);
    let mut g = (0, 0);
    for (r, row) in fld.iter().enumerate() {
        for (c, &x) in row.iter().enumerate() {
            match x {
                'S' => {
                    s = (r, c);
                }
                'G' => {
                    g = (r, c);
                }
                _ => (),
            }
        }
    }
    let dist = bfs(&fld, s);
    let ans = dist[g.0][g.1];

    println!("{}", if ans == INF { -1 } else { ans });
}

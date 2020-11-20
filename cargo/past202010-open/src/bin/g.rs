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

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        fld: [Chars; n],
    }
    let mut ans = 0;
    for r in 0..n {
        for c in 0..m {
            ans += is_ok(n, m, &fld, r, c);
        }
    }

    println!("{}", ans);
}

fn is_ok(n: usize, m: usize, fld: &Vec<Vec<char>>, r: usize, c: usize) -> i32 {
    if fld[r][c] == '.' {
        return 0;
    }
    let mut visited = vec![vec![false; m]; n];
    visited[r][c] = true;
    let mut stack = vec![(r, c)];
    let dr = [0, 1, 0, std::usize::MAX];
    let dc = [1, 0, std::usize::MAX, 0];
    while let Some((cr, cc)) = stack.pop() {
        for i in 0..4 {
            let nr = cr.wrapping_add(dr[i]);
            let nc = cc.wrapping_add(dc[i]);
            if nr < n && nc < m && fld[nr][nc] == '.' && !visited[nr][nc] {
                visited[nr][nc] = true;
                stack.push((nr, nc));
            }
        }
    }
    for r in 0..n {
        for c in 0..m {
            if fld[r][c] == '.' && !visited[r][c] {
                return 0;
            }
        }
    }

    1
}

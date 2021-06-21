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

fn dfs(
    fld: &[Vec<char>],
    start: (usize, usize),
    pos: (usize, usize),
    prev_start: bool,
    mut used: Vec<Vec<bool>>,
) -> Option<i32> {
    used[pos.0][pos.1] = true;
    let h = fld.len();
    let w = fld[0].len();
    let mut ret = None;
    for &(dr, dc) in DIRS {
        let (nr, nc) = (pos.0.wrapping_add(dr), pos.1.wrapping_add(dc));
        if (nr, nc) == start && !prev_start {
            ret = ret.max(Some(1));
        }
        if nr < h && nc < w && fld[nr][nc] == '.' && !used[nr][nc] {
            let new_used = used.clone();
            ret = ret.max(dfs(fld, start, (nr, nc), start == pos, new_used).map(|n| n + 1));
        }
    }
    ret
}

fn solve(fld: &[Vec<char>], start: (usize, usize)) -> Option<i32> {
    let h = fld.len();
    let w = fld[0].len();
    dfs(fld, start, start, false, vec![vec![false; w]; h])
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        fld: [Chars; h],
    }
    let ans = (0..h)
        .cartesian_product(0..w)
        .filter(|&(i, j)| fld[i][j] == '.')
        .map(|(i, j)| solve(&fld, (i, j)))
        .max()
        .unwrap();

    println!("{}", ans.unwrap_or(-1));
}

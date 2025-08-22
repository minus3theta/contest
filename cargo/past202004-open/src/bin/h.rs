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

fn find_char(s: &[Vec<char>], c: char) -> Option<(usize, usize)> {
    for (i, row) in s.iter().enumerate() {
        for (j, &ch) in row.iter().enumerate() {
            if ch == c {
                return Some((i, j));
            }
        }
    }
    None
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n],
    }
    let mut dist = vec![vec![vec![INF; 10]; m]; n];
    let start = find_char(&a, 'S').unwrap();
    let goal = find_char(&a, 'G').unwrap();
    dist[start.0][start.1][0] = 0;
    let mut queue = VecDeque::new();
    queue.push_back((start.0, start.1, 0));
    while let Some((i, j, k)) = queue.pop_front() {
        for (di, dj) in [(0, 1), (1, 0), (0, usize::MAX), (usize::MAX, 0)] {
            let ni = i.wrapping_add(di);
            let nj = j.wrapping_add(dj);
            if ni < n && nj < m {
                if dist[i][j][k] + 1 < dist[ni][nj][k] {
                    dist[ni][nj][k] = dist[i][j][k] + 1;
                    queue.push_back((ni, nj, k));
                }
                if let '1'..='9' = a[ni][nj] {
                    let nk = (a[ni][nj] as u8 - b'0') as usize;
                    if k + 1 == nk && dist[i][j][k] + 1 < dist[ni][nj][nk] {
                        dist[ni][nj][nk] = dist[i][j][k] + 1;
                        queue.push_back((ni, nj, nk));
                    }
                }
            }
        }
    }
    let ans = dist[goal.0][goal.1][9];
    println!("{}", if ans == INF { -1 } else { ans });
}

#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

fn solve(board: Vec<u8>, es: &[(usize, usize)]) -> i32 {
    let mut que = VecDeque::new();
    que.push_back(board.clone());
    let mut dist = HashMap::new();
    dist.insert(board, 0);
    let goal = (0..9).collect_vec();
    while let Some(p) = que.pop_front() {
        for &(u, v) in es {
            if p[u] == 8 || p[v] == 8 {
                let mut q = p.clone();
                q.swap(u, v);
                if dist.get(&q).is_none() {
                    let d = dist[&p] + 1;
                    dist.insert(q.clone(), d);
                    if q == goal {
                        break;
                    }
                    que.push_back(q);
                }
            }
        }
    }
    *dist.get(&goal).unwrap_or(&-1)
}

fn main() {
    input! {
        m: usize,
        es: [(Usize1, Usize1); m],
        p: [Usize1; 8],
    }
    let mut board = vec![8; 9];
    for (i, p) in p.into_iter().enumerate() {
        board[p] = i as u8;
    }

    println!("{}", solve(board, &es));
}

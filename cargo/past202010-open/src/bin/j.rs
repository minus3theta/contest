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

const INF: i64 = 1i64 << 58;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        xab: i64,
        xac: i64,
        xbc: i64,
        s: Chars,
        es: [(Usize1, Usize1, i64); m],
    }
    let mut adj = vec![vec![]; n];
    for &(a, b, c) in &es {
        adj[a].push((b, c));
        adj[b].push((a, c));
    }
    let mut dist = vec![INF; n];
    dist[0] = 0;
    let mut rev_dist = vec![INF; n];
    rev_dist[n - 1] = 0;
    dijkstra(&adj, 0, &mut dist);
    dijkstra(&adj, n - 1, &mut rev_dist);
    assert_eq!(dist[n - 1], rev_dist[0]);

    let mut warp = vec![vec![INF; 3]; 3];
    let mut exist = vec![false; 3];
    for &c in &s {
        exist[c as usize - 'A' as usize] = true;
    }
    if exist[0] && exist[1] {
        warp[0][1] = xab;
        warp[1][0] = xab;
    }
    if exist[0] && exist[2] {
        warp[0][2] = xac;
        warp[2][0] = xac;
    }
    if exist[1] && exist[2] {
        warp[1][2] = xbc;
        warp[2][1] = xbc;
    }
    for k in 0..3 {
        for i in 0..3 {
            for j in 0..3 {
                warp[i][j] = cmp::min(warp[i][j], warp[i][k] + warp[k][j]);
            }
        }
    }
    let mut s_nearest = vec![INF; 3];
    let mut g_nearest = vec![INF; 3];
    for i in 0..n {
        let tp = s[i] as usize - 'A' as usize;
        s_nearest[tp] = cmp::min(s_nearest[tp], dist[i]);
        g_nearest[tp] = cmp::min(g_nearest[tp], rev_dist[i]);
    }
    // dbg!(&warp);
    // dbg!(&s_nearest);
    // dbg!(&g_nearest);
    let mut ans = dist[n - 1]; // no warp
    assert!(ans < INF);
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(warp[i][j], warp[j][i]);
            ans = cmp::min(ans, s_nearest[i] + warp[i][j] + g_nearest[j]);
        }
    }
    assert!(ans > 0);

    println!("{}", ans);
}

fn dijkstra(adj: &Vec<Vec<(usize, i64)>>, s: usize, dist: &mut Vec<i64>) {
    let mut que = BinaryHeap::new();
    que.push((cmp::Reverse(0), s));
    while let Some((cmp::Reverse(cost), v)) = que.pop() {
        if cost > dist[v] {
            continue;
        }
        for &(u, c) in &adj[v] {
            let d = dist[v] + c;
            if d < dist[u] {
                dist[u] = d;
                que.push((cmp::Reverse(dist[u]), u));
            }
        }
    }
}

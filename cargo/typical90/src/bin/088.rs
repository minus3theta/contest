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

fn dfs(
    a: &[i32],
    adj: &[Vec<usize>],
    set: BTreeSet<usize>,
    index: usize,
    hist: &mut BTreeMap<i32, BTreeSet<usize>>,
) -> Option<(BTreeSet<usize>, BTreeSet<usize>)> {
    if index >= a.len() {
        return None;
    }
    let can_append = || {
        for &i in &adj[index] {
            if set.contains(&i) {
                return false;
            }
        }
        true
    };
    let weight = |set: &BTreeSet<usize>| set.iter().map(|&i| a[i]).sum::<i32>();
    if can_append() {
        let mut set = set.clone();
        set.insert(index);
        let w = weight(&set);
        if let Some(prev) = hist.get(&w) {
            return Some((prev.clone(), set));
        }
        hist.insert(w, set.clone());
        if let Some(r) = dfs(a, adj, set, index + 1, hist) {
            return Some(r);
        }
    }
    dfs(a, adj, set, index + 1, hist)
}

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [i32; n],
        xy: [(Usize1, Usize1); q],
    }
    let mut adj = vec![vec![]; n];
    for &(x, y) in &xy {
        adj[y].push(x);
    }
    let mut hist = BTreeMap::new();
    let (b, c) = dfs(&a, &adj, BTreeSet::new(), 0, &mut hist).unwrap();
    fn print(s: &BTreeSet<usize>) {
        println!("{}", s.len());
        for &x in s {
            print!("{} ", x + 1);
        }
        println!();
    }
    print(&b);
    print(&c);
}

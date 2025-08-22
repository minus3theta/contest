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
        a: [i32; 1 << n],
    }
    let mut score = vec![0; 1 << n];
    let mut players = a.into_iter().enumerate().collect::<Vec<_>>();
    for i in 1..=n {
        let mut next_players = Vec::new();
        for (&(xi, x), &(yi, y)) in players.iter().tuples() {
            score[xi] = i;
            score[yi] = i;
            if x > y {
                next_players.push((xi, x));
            } else {
                next_players.push((yi, y));
            }
        }
        players = next_players;
    }
    for &s in &score {
        println!("{}", s);
    }
}

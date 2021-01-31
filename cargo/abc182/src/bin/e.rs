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

#[derive(Debug, Copy, Clone)]
enum Cell {
    Blank,
    Light,
    Block,
}

use Cell::*;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        m: usize,
        light: [(usize, usize); n],
        block: [(usize, usize); m],
    }
    let mut field = vec![vec![Blank; w + 2]; h + 2];
    for &(a, b) in &light {
        field[a][b] = Light;
    }
    for &(c, d) in &block {
        field[c][d] = Block;
    }
    for c in 0..w + 2 {
        field[0][c] = Block;
        field[h + 1][c] = Block;
    }
    for r in 0..h + 2 {
        field[r][0] = Block;
        field[r][w + 1] = Block;
    }
    // dbg!(&field);
    let mut bright = vec![vec![false; w + 2]; h + 2];
    for i in 1..=h {
        let mut light_exist = false;
        let mut begin = 0;
        for end in 0..w + 2 {
            match &field[i][end] {
                Blank => (),
                Light => {
                    light_exist = true;
                }
                Block => {
                    if light_exist {
                        for j in begin..end {
                            bright[i][j] = true;
                        }
                    }
                    light_exist = false;
                    begin = end + 1;
                }
            }
        }
    }
    for i in 1..=w {
        let mut light_exist = false;
        let mut begin = 0;
        for end in 0..h + 2 {
            match &field[end][i] {
                Blank => (),
                Light => {
                    light_exist = true;
                }
                Block => {
                    if light_exist {
                        for j in begin..end {
                            bright[j][i] = true;
                        }
                    }
                    light_exist = false;
                    begin = end + 1;
                }
            }
        }
    }
    // for row in &bright {
    //     for &b in row {
    //         eprint!("{}", if b { '*' } else { '.' });
    //     }
    //     eprintln!();
    // }
    let ans: usize = bright
        .iter()
        .map(|r| r.iter().filter(|&&b| b).count())
        .sum();

    println!("{}", ans);
}

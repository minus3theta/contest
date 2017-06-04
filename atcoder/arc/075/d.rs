#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::cmp;
use std::collections::LinkedList;

#[allow(dead_code)]
fn getline() -> LinkedList<String> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().unwrap();
    s.split(' ').map(|x| x.trim().to_string()).collect::<LinkedList<String>>()
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>(l: &mut LinkedList<String>) -> T {
    l.pop_front().and_then(|s| s.parse().ok()).unwrap()
}

fn main() {
    let mut l = getline();
    let n: usize = get(&mut l);
    let a: i64 = get(&mut l);
    let b: i64 = get(&mut l);
    let c = a - b;
    let hs: Vec<i64> = (0..n).map(|_| get(&mut getline())).collect();
    let mut l = 0i64;
    let mut r = 1e9 as i64;
    while l + 1 != r {
        let m = (l + r) / 2;
        if possible(c, b, &hs, m) {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", r);
}

fn possible(c: i64, b: i64, hs: &Vec<i64>, m: i64) -> bool {
    let sum = hs.iter().map(|h| {
        let rem = cmp::max(0, h - m * b);
        (rem + c - 1) / c
    }).fold(0, |sum, x| sum + x);
    sum <= m
}

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
}

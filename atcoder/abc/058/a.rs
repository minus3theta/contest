#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::cmp;

#[allow(dead_code)]
fn getline() -> Vec<String> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().unwrap();
    s.split(' ').map(|x| x.trim().to_string()).collect::<Vec<String>>()
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>(s: &String) -> T {
    s.parse().ok().unwrap()
}

fn main() {
    let abc = getline();
    let a: i32 = get(&abc[0]);
    let b: i32 = get(&abc[1]);
    let c: i32 = get(&abc[2]);
    if c - b == b - a {
        println!("YES");
    } else {
        println!("NO");
    }
}

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
    let o = getline();
    let o: Vec<char> = o[0].chars().collect();
    let e = getline();
    let e: Vec<char> = e[0].chars().collect();
    for i in 0..e.len() {
        print!("{}{}", o[i], e[i]);
    }
    if e.len() < o.len() {
        print!("{}", o.last().unwrap());
    }
    println!("");
}

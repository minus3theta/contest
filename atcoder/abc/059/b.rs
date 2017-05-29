#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::cmp::*;

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

fn solve() -> String {
    let a = &getline()[0].chars().collect::<Vec<_>>();
    let b = &getline()[0].chars().collect::<Vec<_>>();
    match a.len().cmp(&b.len()) {
        Ordering::Greater => "GREATER".to_string(),
        Ordering::Less => "LESS".to_string(),
        Ordering::Equal => {
            for i in 0..a.len() {
                match a[i].cmp(&b[i]) {
                    Ordering::Greater => return "GREATER".to_string(),
                    Ordering::Less => return "LESS".to_string(),
                    Ordering::Equal => ()
                }
            }
            "EQUAL".to_string()
        }
    }
}
        
fn main() {
    println!("{}", solve());
}

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
    let abc = getline().into_iter()
        .map(|s| s.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    for i in 0..2 {
        if abc[i].last() != abc[i+1].first() {
            println!("NO");
            return;
        }
    }
    println!("YES");
}

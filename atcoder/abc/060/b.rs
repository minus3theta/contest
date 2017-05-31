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
    let a: usize = get(&abc[0]);
    let b: usize = get(&abc[1]);
    let c: usize = get(&abc[2]);
    let mut v = vec![false; b];
    let mut x = 0usize;
    for _ in 0..b {
        v[x] = true;
        x = (x + a) % b;
    }
    if v[c] {
        println!("YES");
    } else {
        println!("NO");
    }
}

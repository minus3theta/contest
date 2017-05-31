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
    let _ = getline();
    let aa = getline();
    let mut aa: Vec<i32> = aa.iter().map(get).collect();
    aa.sort();
    let mut k = 0;
    let mut prev = -1;
    for x in aa {
        if x != prev {
            k += 1;
            prev = x;
        }
    }
    if k % 2 == 0 {
        k -= 1;
    }
    println!("{}", k);
}

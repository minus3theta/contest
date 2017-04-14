#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::cmp;

fn read_vec() -> Vec<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.split(' ').map(|x| x.to_string()).collect::<Vec<String>>()
}

fn main() {
    let n: i32 = read_vec()[0].trim().parse().expect("");
    println!("{}", 800 * n - 200 * (n / 15));
}

#[allow(unused_imports)]
use std::io;
use std::io::stdin;
// use std::cmp;

fn read_line_as_vec() -> Vec<String> {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("");
    s.split(' ').map(|x| x.to_string()).collect::<Vec<String>>()
}

fn main() {
    let ab = read_line_as_vec();
    let a = ab[0].trim();
    let b = ab[1].trim();
    let t = if a == b { "H" } else { "D" };
    println!("{}", t);
}

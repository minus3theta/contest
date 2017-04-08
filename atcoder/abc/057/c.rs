#[allow(unused_imports)]
use std::io;
use std::io::stdin;
use std::cmp;

fn read_line_as_vec() -> Vec<String> {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("");
    s.split(' ').map(|x| x.to_string()).collect::<Vec<String>>()
}

fn digit(x: u64) -> u32 {
    let mut d: u32 = 1;
    while 10u64.pow(d) <= x {
        d += 1;
    }
    d
}

fn main() {
    let n: u64 = read_line_as_vec()[0].trim().parse().expect("");
    let mut a: u64 = 1;
    let mut min_d: u32 = u32::max_value();
    while a * a <= n {
        if n % a != 0 {
            a += 1;
            continue;
        }
        min_d = cmp::min(min_d, digit(n / a));
        a += 1;
    }
    println!("{}", min_d);
}

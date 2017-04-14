use std::io;
#[allow(unused_imports)]
use std::cmp;
use std::collections::LinkedList;

fn read_list() -> LinkedList<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("");
    s.split(' ').map(|x| x.to_string()).collect::<LinkedList<String>>()
}

fn pop_parse<T: std::str::FromStr>(list: &mut LinkedList<String>) -> T {
    list.pop_front().expect("").trim().parse::<T>().ok().unwrap()
}

fn main() {
    let mut nm = read_list();
    let n: i64 = pop_parse(&mut nm);
    let m: i64 = pop_parse(&mut nm);
    let simple = cmp::min(n, m / 2);
    let m = m - simple * 2;
    println!("{}", simple + m / 4);
}

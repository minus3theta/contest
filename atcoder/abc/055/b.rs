#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
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
    let modulo = 1e9 as i64 + 7;
    let mut n = read_list();
    let n: i64 = pop_parse(&mut n);
    let mut pow:i64 = 1;
    for i in 1..n+1 {
        pow = (pow * i) % modulo;
    }
    println!("{}", pow);
}

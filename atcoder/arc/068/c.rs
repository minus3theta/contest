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
    let x: i64 = pop_parse(&mut read_list());
    let y = (x + 10) / 11;
    let z = if y * 11 - 5 >= x {y * 2 - 1} else {y * 2};
    println!("{}", z);
}

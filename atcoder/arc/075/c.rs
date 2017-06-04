#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::cmp;
use std::collections::LinkedList;

#[allow(dead_code)]
fn getline() -> LinkedList<String> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().unwrap();
    s.split(' ').map(|x| x.trim().to_string()).collect::<LinkedList<String>>()
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>(l: &mut LinkedList<String>) -> T {
    l.pop_front().and_then(|s| s.parse().ok()).unwrap()
}

fn main() {
    let n: i32 = get(&mut getline());
    let mut ss: Vec<i32> = (0..n).map(|_| get(&mut getline())).collect();
    ss.sort();
    let mut sum = ss.iter().fold(0, |sum, x| sum + x);
    for &x in ss.iter() {
        if sum % 10 != 0 {
            break;
        }
        if x % 10 != 0 {
            sum -= x;
            break;
        }
    }
    if sum % 10 == 0 {
        println!("0");
    } else {
        println!("{}", sum);
    }
}

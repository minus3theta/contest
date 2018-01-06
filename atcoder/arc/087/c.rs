#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::cmp;
use std::collections::LinkedList;

#[allow(dead_code)]
fn getline() -> LinkedList<String> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().unwrap();
    s.split(' ')
        .map(|x| x.trim().to_string())
        .collect::<LinkedList<String>>()
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>(l: &mut LinkedList<String>) -> T {
    l.pop_front().and_then(|s| s.parse().ok()).unwrap()
}

fn main() {
    let _ = getline();
    let mut s = String::new();
    io::stdin().read_line(&mut s).unwrap();
    let mut v: Vec<i32> = s.split(' ').map(|x| x.trim().parse().unwrap()).collect();
    v.sort();
    let mut prev: i32 = 0;
    let mut prev_count: i32 = 0;
    let mut ans: i32 = 0;
    for x in v {
        if x == prev {
            prev_count += 1;
        } else {
            if prev < prev_count {
                ans += prev_count - prev;
            } else if prev > prev_count {
                ans += prev_count;
            }
            prev = x;
            prev_count = 1;
        }
    }
    if prev < prev_count {
        ans += prev_count - prev;
    } else if prev > prev_count {
        ans += prev_count;
    }
    println!("{}", ans);
}

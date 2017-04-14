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

fn align(s: &Vec<bool>, a: bool, b: bool) -> Option<Vec<bool>> {
    let mut ans = vec![a, b];
    for i in 1..s.len() {
        let next = ans[i-1] ^ ans[i] ^ s[i];
        ans.push(next);
    }
    if ans.pop().unwrap() == a && ans[s.len()-1] ^ a ^ s[0] == b {
        Some(ans)
    } else {
        None
    }
}

fn main() {
    let _: i32 = pop_parse(&mut read_list());
    let s: String = pop_parse(&mut read_list());
    let vs: Vec<bool> = s.into_bytes().into_iter()
        .map(|x| match x as char {
            'o' => true,
            'x' => false,
            _   => panic!(),
        }).collect();
    let mut ans: Option<Vec<bool>> = None;
    for (a,b) in vec![(true,true), (true,false), (false,true), (false,false)] {
        match align(&vs, a, b) {
            Some(v) => ans = Some(v),
            None => {},
        }
    }
    let ans = match ans {
        Some(v) => String::from_utf8(
            v.into_iter().map(|x| if x {'S' as u8} else {'W' as u8}).collect()).unwrap(),
        None => String::from("-1"),
    };
    println!("{}", ans);
}

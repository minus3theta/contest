#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

use regex::Regex;

#[fastout]
fn main() {
    input! {
        mut s: String,
    }
    let re = Regex::new(r"\(([a-z]*)\)").unwrap();
    while let Some(c) = re.captures(&s) {
        let a = c.get(1).unwrap().as_str();
        let x = a.chars().chain(a.chars().rev()).collect::<String>();
        s = re.replace(&s, x.as_str()).to_string();
    }

    println!("{}", s);
}

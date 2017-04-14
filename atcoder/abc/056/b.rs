#[allow(unused_imports)]
use std::io;
use std::io::stdin;

fn read_line_as_vec() -> Vec<String> {
    let mut s = String::new();
    stdin().read_line(&mut s).expect("");
    s.split(' ').map(|x| x.to_string()).collect::<Vec<String>>()
}

fn main() {
    let wab = read_line_as_vec();
    let w: u32 = wab[0].trim().parse().expect("");
    let mut a: u32 = wab[1].trim().parse().expect("");
    let mut b: u32 = wab[2].trim().parse().expect("");
    if a > b {
        std::mem::swap(&mut a, &mut b);
    }
    println!("{}", b.saturating_sub(a + w));
}

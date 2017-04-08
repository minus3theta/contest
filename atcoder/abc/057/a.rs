use std::io;

fn main() {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Read failed");
    let ab: Vec<&str> = s.split(' ').collect();
    let a: i32 = ab[0].trim().parse().expect("");
    let b: i32 = ab[1].trim().parse().expect("");
    let ans = (a + b) % 24;
    println!("{}", ans);
}

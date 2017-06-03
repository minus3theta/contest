#[allow(unused_imports)]
use std::io;
#[allow(unused_imports)]
use std::cmp;

#[allow(dead_code)]
fn getline() -> Vec<String> {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok().unwrap();
    s.split(' ').map(|x| x.trim().to_string()).collect::<Vec<String>>()
}

#[allow(dead_code)]
fn get<T: std::str::FromStr>(s: &String) -> T {
    s.parse().ok().unwrap()
}

fn main() {
    let modulo = 1e9 as usize + 7;
    let n: usize = get(&getline()[0]);
    let mut is_prime = vec![true; n+1];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..n+1 {
        if !is_prime[i] {
            continue;
        }
        for j in 2.. {
            if i * j > n {
                break;
            }
            is_prime[i * j] = false;
        }
    }
    let primes: Vec<_> = (0..n+1).filter(|&x| is_prime[x]).collect();
    let mut count_prime = vec![1; n+1];
    for i in 1..n+1 {
        let mut x = i;
        for &p in primes.iter() {
            if x == 1 {
                break;
            }
            while x % p == 0 {
                count_prime[p] += 1;
                x /= p;
            }
        }
    }
    let ans = count_prime.iter()
        .fold(1, |prod, &x| (prod * x) % modulo);
    println!("{}", ans);
}

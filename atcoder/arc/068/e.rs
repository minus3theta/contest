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

struct Bit<T, F> {
    n: usize,
    dat: Vec<T>,
    op: F,
    unit: T,
}

impl<T: Clone, F: Fn(&mut T, &T)> Bit<T, F> {
    #[allow(dead_code)]
    fn new(n: usize, op: F, unit: T) -> Self {
        Bit {
            n: n,
            dat: vec![unit.clone(); n + 1],
            op: op,
            unit: unit,
        }
    }
    #[allow(dead_code)]
    fn from_vec(mut v: Vec<T>, op: F, unit: T) -> Self {
        let n = v.len();
        let mut dat = vec![unit.clone()];
        dat.append(&mut v);
        for i in 1..n {
            let j = i as i32;
            let b = (j & -j) as usize;
            let x = dat[i].clone();
            op(&mut dat[i + b], &x);
        }
        Bit {
            n: n,
            dat: dat,
            op: op,
            unit: unit,
        }
    }
    fn operate(&mut self, k: usize, a: &T) {
        let mut k = k;
        while k <= self.n {
            (self.op)(&mut self.dat[k], &a);
            let l = k as i32;
            k += (l & -l) as usize;
        }
    }
    fn accum(&self, k: usize) -> T {
        let mut k = k;
        let mut sum = self.unit.clone();
        while k > 0 {
            (self.op)(&mut sum, &self.dat[k]);
            let l = k as i32;
            k -= (l & -l) as usize;
        }
        sum
    }
}

fn main() {
    let nm = getline();
    let n: i32 = get(&nm[0]);
    let m: usize = get(&nm[1]);
    let mut lrs: Vec<Vec<(i32,i32)>> = vec![vec![]; m + 1];
    for _ in 0..n {
        let lr = getline();
        let l: i32 = get(&lr[0]);
        let r: i32 = get(&lr[1]);
        lrs[(r - l + 1) as usize].push((l, r + 1));
    }
    let mut total = 0;
    let mut bit = Bit::new(m, |x, y| *x += *y, 0);
    for d in 1..m+1 {
        let mut ans = n;
        ans -= total;
        total += lrs[d].len() as i32;
        for i in 1.. {
            let x = i * d;
            if x > m {
                break;
            }
            ans += bit.accum(x);
        }
        for &(l, r) in &lrs[d] {
            bit.operate(l as usize, &1);
            bit.operate(r as usize, &-1);
        }
        println!("{}", ans);
    }
}

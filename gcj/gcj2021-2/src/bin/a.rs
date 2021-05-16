// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, [ $t:tt ]) => {{
        let len = read_value!($iter, usize);
        (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    }};

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn flush() {
    use std::io::{stdout, Write};
    stdout().flush().unwrap();
}

fn line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn solve(n: usize) {
    for i in 1..n {
        println!("M {} {}", i, n);
        flush();
        let argmin: usize = line().trim().parse().unwrap();
        if argmin != i {
            println!("S {} {}", i, argmin);
            flush();
            let _ = line();
        }
    }
    println!("D");
    let _ = line();
}

fn main() {
    let s = {
        // use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s
    };
    let mut iter = s.split_whitespace();
    input_inner! {
        iter,
        t: usize,
        n: usize,
    }
    for _ in 1..=t {
        solve(n);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn cost() {
//         let c = 100_000_000;
//         let mut cost = 0;
//         for i in 2..=100 {
//             cost += c / i;
//         }
//         println!("{}", cost);
//         assert!(false);
//     }
// }

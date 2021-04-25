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

fn put(x: usize) {
    println!("{}", x);
    use std::io::{stdout, Write};
    stdout().flush().unwrap();
}

fn eval(pile: &Vec<Vec<i32>>, b: usize, place: usize, digit: i32) -> i64 {
    let h = pile[place].len();
    if h == b {
        -(1 << 60)
    } else {
        match digit {
            9 => 1 << (h + 10),
            _ => {
                if h == b - 1 {
                    0
                } else {
                    100 - place as i64
                }
            }
        }
    }
}

fn solve(n: usize, b: usize) -> bool {
    let mut pile = vec![vec![]; n];
    for _ in 0..n * b {
        let d = line();
        let d: i32 = d.trim().parse().unwrap();
        if d < 0 {
            return false;
        }
        let place = (0..n).max_by_key(|&p| eval(&pile, b, p, d)).unwrap();
        // dbg!((d, place));
        pile[place].push(d);
        put(place + 1);
    }

    true
}

fn line() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
    s
}

fn main() {
    let s = line();
    let mut iter = s.split_whitespace();
    input_inner! {
        iter,
        t: usize,
        n: usize,
        b: usize,
        _p: i64,
    }
    for _ in 1..=t {
        if !solve(n, b) {
            break;
        }
    }
}

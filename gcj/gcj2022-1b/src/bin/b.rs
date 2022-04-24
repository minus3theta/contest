use std::collections::HashMap;

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };

}
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

fn solve(x: Vec<Vec<i64>>) -> i64 {
    let mut min_count = HashMap::new();
    min_count.insert(0, 0);
    for cus in &x {
        let &min = cus.iter().min().unwrap();
        let &max = cus.iter().max().unwrap();
        let mut next = HashMap::new();
        for (last, cost) in min_count {
            let mut update = |begin: i64, end: i64| {
                let ent = next.entry(end).or_insert(1i64 << 60);
                *ent = (cost + (last - begin).abs() + (end - begin).abs()).min(*ent);
            };
            update(min, max);
            update(max, min);
        }
        min_count = next;
    }
    *min_count.values().min().unwrap()
}

fn main() {
    let s = {
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };
    let mut iter = s.split_ascii_whitespace();
    input_inner! {
        iter,
        t: usize,
    }
    for i in 0..t {
        input_inner! {
            iter,
            n: usize,
            p: usize,
            x: [[i64; p]; n],
        }
        println!("Case #{}: {}", i + 1, solve(x));
    }
}

use std::io::{stdout, Write};

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
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

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        // dbg!(i + 1);

        println!("00000000");
        let mut shift = 2;
        let mut iter = 0;
        loop {
            stdout().flush().unwrap();
            input! {
                x: i32,
            }
            // dbg!(x);
            if x == 0 {
                break;
            }
            if x < 0 {
                return;
            }

            if x == 8 {
                println!("11111111");
            } else if x % 2 != 0 {
                println!("00000001");
            } else {
                if x == 2 {
                    iter += 1;
                    if iter > 10 {
                        shift <<= 1;
                        if shift >= 0b10000000 {
                            shift = 2;
                        }
                    }
                }
                println!("{:08b}", 1 | shift);
            }
        }
    }
}

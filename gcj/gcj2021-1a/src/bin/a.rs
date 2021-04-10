use std::iter::repeat;

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

fn increment(v: &mut Vec<char>) {
    let n = v.len();
    if v.iter().all(|&c| c == '9') {
        *v = Some('1').into_iter().chain(repeat('0').take(n)).collect();
    } else {
        for i in (0..n).rev() {
            if v[i] == '9' {
                v[i] = '0';
            } else {
                v[i] = (v[i] as u8 + 1) as char;
                break;
            }
        }
    }
}

fn step(x: &Vec<char>, y: &mut Vec<char>) -> i64 {
    let mut op = 0;
    if !x.starts_with(&y) {
        let count = x.len().saturating_sub(y.len());
        y.extend(repeat('0').take(count));
        op += count as i64;
        if x.len() == y.len() && x >= y {
            y.push('0');
            op += 1;
        }
        return op;
    }
    let mut z = x.clone();
    increment(&mut z);
    if z.starts_with(&y) {
        op = z.len() as i64 - y.len() as i64;
        *y = z;
        return op;
    }
    let count = x.len().saturating_sub(y.len());
    y.extend(repeat('0').take(count));
    op += count as i64;
    if x.len() == y.len() && x >= y {
        y.push('0');
        op += 1;
    }
    op
}

fn solve(x: &mut [Vec<char>]) -> i64 {
    let n = x.len();
    let mut op = 0;
    for i in 0..n - 1 {
        let mut y = x[i + 1].clone();
        op += step(&x[i], &mut y);
        x[i + 1] = y;
    }
    op
}

fn main() {
    let s = {
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };
    let mut iter = s.split_whitespace();
    input_inner! {
        iter,
        t: usize,
    }
    for c in 1..=t {
        input_inner! {
            iter,
            n: usize,
            x: [chars; n],
        }
        let mut x = x;
        println!("Case #{}: {}", c, solve(&mut x));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn step1() {
        let x = "123".chars().collect();
        let mut y = "1".chars().collect();
        assert_eq!(step(&x, &mut y), 2);
        assert_eq!(y, "124".chars().collect::<Vec<_>>());
    }

    // #[test]
    // fn step2() {
    //     let x = 123;
    //     let mut y = 130;
    //     assert_eq!(step(x, &mut y), 0);
    //     assert_eq!(y, 130);
    // }

    // #[test]
    // fn step3() {
    //     let x = 100;
    //     let mut y = 7;
    //     assert_eq!(step(x, &mut y), 2);
    //     assert_eq!(y, 700);
    // }

    #[test]
    fn step4() {
        let x = "700".chars().collect();
        let mut y = "10".chars().collect();
        assert_eq!(step(&x, &mut y), 2);
        assert_eq!(y, "1000".chars().collect::<Vec<_>>());
    }

    // #[test]
    // fn step5() {
    //     let x = 199;
    //     let mut y = 1;
    //     assert_eq!(step(x, &mut y), 3);
    //     assert_eq!(y, 1000);
    // }

    #[test]
    fn step6() {
        let x = "4".chars().collect();
        let mut y = "19".chars().collect();
        assert_eq!(step(&x, &mut y), 0);
        assert_eq!(y, "19".chars().collect::<Vec<_>>());
    }
}

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

fn solve(l: &mut [usize]) -> usize {
    let n = l.len();
    let mut cost = 0;
    for i in 0..(n - 1) {
        let (j, _) = l.iter().enumerate().skip(i).find(|(_, &x)| x == i).unwrap();
        cost += j - i + 1;
        l[i..=j].reverse();
    }
    cost
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
            l: [usize1; n],
        }
        let mut l = l;

        println!("Case #{}: {}", c, solve(&mut l));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        assert_eq!(solve(&mut [3, 1, 0, 2]), 6);
    }
    #[test]
    fn case2() {
        assert_eq!(solve(&mut [1, 0]), 2);
    }
    #[test]
    fn case3() {
        assert_eq!(solve(&mut [1, 2, 0]), 5);
    }
}

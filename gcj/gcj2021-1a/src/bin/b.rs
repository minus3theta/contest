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

type Pile = [(i64, usize)];

use std::collections::BTreeMap;

fn factor(mut x: i64, pile: &Pile) -> Option<BTreeMap<i64, usize>> {
    let mut f = BTreeMap::new();
    for &(d, _) in pile {
        while x % d == 0 {
            *f.entry(d).or_insert(0) += 1;
            x /= d;
        }
    }
    if x != 1 {
        None
    } else {
        Some(f)
    }
}

fn solve(pile: &Pile) -> i64 {
    let sum: i64 = pile.iter().map(|&(p, n)| p * n as i64).sum();
    for prod in (sum.saturating_sub(30000).max(1)..=sum).rev() {
        if let Some(f) = factor(prod, pile) {
            if pile.iter().all(|&(p, n)| f.get(&p).unwrap_or(&0) <= &n) {
                if sum - f.iter().map(|(&p, &n)| p * n as i64).sum::<i64>() == prod {
                    return prod;
                }
            }
        }
    }
    0
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
            m: usize,
            x: [(i64, usize); m],
        }
        println!("Case #{}: {}", c, solve(&x));
    }
}

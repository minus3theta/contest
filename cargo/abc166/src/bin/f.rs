#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;

fn index(s: &str) -> (usize, usize) {
  match s {
    "AB" => (0, 1),
    "AC" => (0, 2),
    _ => (1, 2),
  }
}

fn solve(abc: &mut Vec<i64>, s: &Vec<String>) -> Option<Vec<char>> {
  if abc.iter().sum::<i64>() == 0 {
    return None;
  }
  let mut ans = vec![];
  for i in 0..s.len() {
    let (l, r) = index(&s[i]);
    let (al, ar) = (abc[l], abc[r]);
    let mut pass = |src: usize, dst: usize| {
      ans.push(['A', 'B', 'C'][dst]);
      abc[src] -= 1;
      abc[dst] += 1;
    };
    match (al, ar) {
      (0, 0) => return None,
      (1, 1) => {
        if i == s.len() - 1 || s[i] == s[i + 1] {
          pass(l, r);
        } else {
          let (nl, nr) = index(&s[i + 1]);
          if l == nl || l == nr {
            pass(r, l);
          } else {
            pass(l, r);
          }
        }
      }
      (al, ar) if al > ar => {
        pass(l, r);
      }
      _ => {
        pass(r, l);
      }
    }
  }

  Some(ans)
}

#[fastout]
fn main() {
  input! {
    n: usize,
    mut abc: [i64; 3],
    s: [String; n],
  }
  match solve(&mut abc, &s) {
    None => {
      println!("No");
    }
    Some(v) => {
      println!("Yes");
      for c in &v {
        println!("{}", c);
      }
    }
  }
}

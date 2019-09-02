#[allow(unused_imports)]
use std::cmp;
use std::io::Write;

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

  ($iter:expr, $t:ty) => {
    $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

type Index = (usize, usize);

fn get_index(i: usize, j: usize) -> Index {
  if i < j { (i, j) } else { (j, i) }
}

use std::collections::{BTreeMap, BTreeSet};

fn longest_path(v: Index, adj: &BTreeMap<Index, Vec<Index>>, len: &mut BTreeMap<Index, usize>, active: &mut BTreeSet<Index>) -> Option<usize> {
  if let Some(l) = len.get(&v) {
    return Some(*l);
  }
  if active.contains(&v) {
    return None;
  }
  let mut l = 0;
  active.insert(v);
  if let Some(adj_v) = adj.get(&v) {
    for &next in adj_v {
      if let Some(nl) = longest_path(next, adj, len, active) {
        l = cmp::max(l, nl + 1);
      } else {
        return None;
      }
    }
  }
  len.insert(v, l);
  active.remove(&v);
  Some(l)
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    a: [[usize1; n-1]; n],
  }
  let mut adj = BTreeMap::new();
  for i in 0 .. n {
    for j in 0 .. n - 2 {
      let index = get_index(i, a[i][j]);
      let next = adj.entry(index).or_insert(vec![]);
      next.push(get_index(i, a[i][j+1]))
    }
  }
  let mut len = BTreeMap::new();
  let mut ans = 0;
  let mut active = BTreeSet::new();
  for &idx in adj.keys() {
    if let Some(l) = longest_path(idx, &adj, &mut len, &mut active) {
      ans = cmp::max(ans, l as i64 + 1);
    } else {
      ans = -1;
      break;
    }
  }

  puts!("{}\n", ans);
}

#[allow(unused_imports)]
use std::cmp;

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
  (source = $s:expr, $($r:tt)*) => {
    let mut iter = $s.split_whitespace();
    input_inner!{iter, $($r)*}
  };
  ($($r:tt)*) => {
    #[allow(unused_mut)]
    let mut s = {
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

struct Reroot<ATTR, RET, MON, MAP, ADD, APPLY> {
  vs: Vec<(ATTR, Vec<usize>)>,
  child: Vec<Vec<usize>>,
  sub: Vec<Option<RET>>,
  total: Vec<Option<RET>>,
  unit: MON,
  map: MAP,
  add: ADD,
  apply: APPLY,
}

impl <ATTR, RET, MON, MAP, APPLY, ADD> Reroot<ATTR, RET, MON, MAP, ADD, APPLY> where
  RET: std::fmt::Debug,
  MON: Clone + std::fmt::Debug,
  MAP: Fn(&RET) -> MON,
  APPLY: Fn(&ATTR, MON) -> RET,
  ADD: Fn(&MON, &MON) -> MON
{
  fn new(vs: Vec<(ATTR, Vec<usize>)>, unit: MON, map: MAP, add: ADD, apply: APPLY) -> Self {
    let child = vs.iter().map(|_| Vec::new()).collect();
    let sub = vs.iter().map(|_| None).collect();
    let total = vs.iter().map(|_| None).collect();
    Reroot {
      vs: vs,
      child: child,
      sub: sub,
      total: total,
      unit: unit,
      map: map,
      add: add,
      apply: apply,
    }
  }
  fn solve(&mut self) {
    self.bottom_up(0, None);
    self.top_down(0, None);
  }
  fn bottom_up(&mut self, v: usize, prev: Option<usize>) {
    let mut m = self.unit.clone();
    for u in self.vs[v].1.clone() {
      if let Some(prev) = prev {
        if u == prev {
          continue;
        }
      }
      self.child[v].push(u);
      self.bottom_up(u, Some(v));
      if let &Some(ref r) = &self.sub[u] {
        m = (self.add)(&m, &(self.map)(r));
      } else {
        unreachable!()
      }
    }
    self.sub[v] = Some((self.apply)(&self.vs[v].0, m));
  }
  fn top_down(&mut self, v: usize, par: Option<RET>) {
    let mut left_scan = vec![self.unit.clone()];
    for &u in self.child[v].iter() {
      if let &Some(ref r) = &self.sub[u] {
        let x = (self.add)(&left_scan.iter().last().unwrap(), &(self.map)(r));
        left_scan.push(x);
      } else {
        unreachable!()
      }
    }
    let mut right_scan = vec![self.unit.clone()];
    for &u in self.child[v].iter().rev() {
      if let &Some(ref r) = &self.sub[u] {
        let x = (self.add)(&right_scan.iter().last().unwrap(), &(self.map)(r));
        right_scan.push(x);
      } else {
        unreachable!()
      }
    }
    let right_scan: Vec<_> = right_scan.into_iter().rev().collect();
    let par_mon = if let Some(par) = par {
      (self.map)(&par)
    } else {
      self.unit.clone()
    };
    let val = (self.apply)(&self.vs[v].0, (self.add)(&left_scan.iter().last().unwrap(), &par_mon));
    for (i, u) in self.child[v].clone().into_iter().enumerate() {
      let next_par = Some((self.apply)(&self.vs[v].0, (self.add)(&par_mon, &(self.add)(&left_scan[i], &right_scan[i+1]))));
      self.top_down(u, next_par);
    }
    self.total[v] = Some(val);
  }
}

fn main() {
  input! {
    n: usize,
    m: i64,
    xys: [(usize1, usize1); n-1]
  }
  let mut vs = vec![((), vec![]); n];
  for &(x, y) in xys.iter() {
    vs[x].1.push(y);
    vs[y].1.push(x);
  }
  let mut graph = Reroot::new(vs, 1i64, |x| x+1, |x, y| x * y % m, |_, x| x);
  graph.solve();
  for x in graph.total.iter() {
    println!("{}", x.unwrap());
  }
}

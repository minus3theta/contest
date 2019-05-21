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
  child: Vec<Vec<(usize, RET)>>,
  unit: MON,
  map: MAP,
  add: ADD,
  apply: APPLY,
}

impl <ATTR, RET, MON, MAP, APPLY, ADD> Reroot<ATTR, RET, MON, MAP, ADD, APPLY> where
  MON: Clone,
  MAP: Fn(RET) -> MON,
  APPLY: Fn(&ATTR, MON) -> RET,
  ADD: Fn(MON, &MON) -> MON
{
  fn new(vs: Vec<(ATTR, Vec<usize>)>, unit: MON, map: MAP, add: ADD, apply: APPLY) -> Self {
    let child = vs.iter().map(|_| Vec::new()).collect();
    Self {
      vs: vs,
      child: child,
      unit: unit,
      map: map,
      add: add,
      apply: apply,
    }
  }
  fn bottom_up(&mut self, v: usize, prev: Option<usize>) {
    unimplemented!()
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
  let mut graph = Reroot::new(vs, 1i64, |x: i64| x+1, |_, _| unimplemented!(), |_, _| unimplemented!());
  println!("");
}

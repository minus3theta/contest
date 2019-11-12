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

  ($iter:expr, [ $t:tt ]) => {{
    let len = read_value!($iter, usize);
    (0..len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
  }};

  ($iter:expr, $t:ty) => {
    $iter.next().unwrap().parse::<$t>().expect("Parse error")
  };
}

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
enum State {
  Fresh,
  Temp,
  Fix,
}

fn find_cycle(v: usize, adj: &Vec<Vec<usize>>, prev: &mut Vec<Option<usize>>, state: &mut Vec<State>) -> Option<usize> {
  match state[v] {
    State::Fresh => {
      state[v] = State::Temp;
      for &next in &adj[v] {
        prev[next] = Some(v);
        if let Some(e) = find_cycle(next, adj, prev, state) {
          return Some(e);
        }
      }
      state[v] = State::Fix;
    }
    State::Temp => {
      return Some(v);
    }
    State::Fix => ()
  }
  None
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    n: usize,
    m: usize,
    es: [(usize1, usize1); m],
  }
  let mut adj = vec![vec![]; n];
  for &(a, b) in &es {
    adj[a].push(b);
  }
  let mut prev = vec![None; n];
  let mut state = vec![State::Fresh; n];
  let mut end = None;
  for i in 0 .. n {
    if let Some(e) = find_cycle(i, &adj, &mut prev, &mut state) {
      end = Some(e);
      break;
    }
  }
  use std::collections::BTreeSet;
  match end {
    Some(end) => {
      let mut vs = BTreeSet::new();
      let mut v = end;
      loop {
        vs.insert(v);
        v = prev[v].unwrap();
        if v == end {
          break;
        }
      }
      let mut es = es;
      loop {
        es = es.into_iter().filter(|&(a, b)| vs.contains(&a) && vs.contains(&b)).collect();
        let rest: Vec<_> = es.iter().filter(|&&(a, b)| prev[b] != Some(a)).collect();
        match rest.first() {
          None => break,
          Some(&&(mut a, b)) => {
            prev[b] = Some(a);
            let mut next_vs = BTreeSet::new();
            next_vs.insert(a);
            while a != b {
              a = prev[a].unwrap();
              next_vs.insert(a);
            }
            vs = next_vs;
          }
        }
      }
      puts!("{}\n", vs.len());
      for &v in &vs {
        puts!("{}\n", v+1);
      }
    }
    None => {
      puts!("-1\n");
    }
  }
}

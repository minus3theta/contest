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

fn next_pos(cs: &Vec<Vec<usize>>, c: usize, pos: usize) -> (usize, bool) {
  let p = match cs[c].binary_search(&pos) {
    Ok(p) => p,
    Err(p) => p,
  };
  if p == cs[c].len() {
    (cs[c][0], true)
  } else {
    (cs[c][p], false)
  }
}

fn main() {
  let out = std::io::stdout();
  let mut out = std::io::BufWriter::new(out.lock());
  macro_rules! puts {
    ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  }
  input! {
    s: chars,
    t: chars,
  }
  let mut cs = vec![vec![]; 26];
  for (i, &c) in s.iter().enumerate() {
    let c = c as usize - 'a' as usize;
    cs[c].push(i);
  }
  for &c in &t {
    let c = c as usize - 'a' as usize;
    if cs[c].is_empty() {
      puts!("-1\n");
      return;
    }
  }
  let mut pos = 0usize;
  for i in 0 .. t.len() {
    let c = t[i] as usize - 'a' as usize;
    let (p, wrap) = next_pos(&cs, c, pos % s.len());
    if wrap {
      pos = (pos / s.len() + 1) * s.len() + p + 1;
    } else {
      pos = pos / s.len() * s.len() + p + 1;
    }
  }

  puts!("{}\n", pos);
}

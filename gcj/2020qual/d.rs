#[allow(unused_imports)]
use std::cmp;

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

fn query(i: usize) -> bool {
  println!("{}", i+1);
  input! {
    v: i8,
  }
  v == 1
}

fn answer(mut arr: Vec<Option<bool>>, flipped: bool, reversed: bool) -> bool {
  let mut ans = String::new();
  if reversed {
    arr.reverse();
  }
  for &x in &arr {
    if flipped == x.unwrap() {
      ans.push('0');
    } else {
      ans.push('1');
    }
  }
  println!("{}", ans);
  input! {
    s: String,
  }
  s == "Y"
}

fn solve(b: usize) -> bool {
  let mut arr = vec![None; b];
  let mut same = None;
  let mut diff = None;
  let mut flipped = false;
  let mut reversed = false;
  let mut head = 0;
  let store = |a: &mut Vec<_>, i, v, f, r| {
    let j = if r {b-i-1} else {i};
    a[j] = Some(f != v);
  };
  while head < b / 2 {
    let mut run = 0;
    if let Some(same) = same {
      let s = query(same);
      run += 1;
      flipped = s != arr[same].unwrap();
    }
    if let Some(diff) = diff {
      let d = query(diff);
      run += 1;
      if flipped {
        reversed = d == arr[diff].unwrap();
      } else {
        reversed = d != arr[diff].unwrap();
      }
    }
    while head < b / 2 && run < 10 {
      let h = query(head);
      run += 1;
      if run >= 10 {
        break;
      }
      let t = query(b-head-1);
      run += 1;
      store(&mut arr, head, h, flipped, reversed);
      store(&mut arr, b-head-1, t, flipped, reversed);
      if h == t {
        if same == None {
          same = Some(head);
        }
      } else {
        if diff == None {
          diff = Some(head)
        }
      }
      head += 1;
    }
  }

  answer(arr, flipped, reversed)
}

fn main() {
  // let out = std::io::stdout();
  // let mut out = std::io::BufWriter::new(out.lock());
  // macro_rules! puts {
  //   ($($format:tt)*) => (write!(out,$($format)*).unwrap());
  // }
  input! {
    t: usize,
    b: usize,
  }
  for _ in 0 .. t {
    if ! solve(b) {
      return;
    }
  }

  // puts!("{}\n", 0);
}

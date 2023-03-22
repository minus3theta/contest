#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use num::Integer;
use num_integer::Roots;
#[allow(unused_imports)]
use proconio::derive_readable;
use proconio::fastout;
use proconio::input;
#[allow(unused_imports)]
use proconio::marker::*;
#[allow(unused_imports)]
use std::cmp;
#[allow(unused_imports)]
use std::collections::*;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
        qs: [(Usize1, usize); q],
    }
    let b = (n / q.sqrt()).max(1);
    let max_a = *a.iter().max().unwrap();
    let mut qs = qs.into_iter().enumerate().collect_vec();
    qs.sort_by_key(|&(_, (l, r))| (l / b, r));

    let mut left = 0;
    let mut right = 0;
    let mut state = State {
        popl: vec![0; max_a + 1],
        value: 0,
    };
    let mut ans = vec![0; q];
    for &(i, (l, r)) in &qs {
        while left > l {
            left -= 1;
            state.insert(a[left]);
        }
        while right < r {
            state.insert(a[right]);
            right += 1;
        }
        while left < l {
            state.remove(a[left]);
            left += 1;
        }
        while right > r {
            right -= 1;
            state.remove(a[right]);
        }
        ans[i] = state.value;
    }
    for &x in &ans {
        println!("{}", x);
    }
}

struct State {
    popl: Vec<usize>,
    value: usize,
}

impl State {
    fn score(count: usize) -> usize {
        count * count.saturating_sub(1) * count.saturating_sub(2) / 6
    }

    fn insert(&mut self, x: usize) {
        self.value -= Self::score(self.popl[x]);
        self.popl[x] += 1;
        self.value += Self::score(self.popl[x]);
    }

    fn remove(&mut self, x: usize) {
        self.value -= Self::score(self.popl[x]);
        self.popl[x] -= 1;
        self.value += Self::score(self.popl[x]);
    }
}

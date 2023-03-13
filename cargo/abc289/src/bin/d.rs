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
#[allow(unused_imports)]
use std::collections::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Open,
    Possible,
    Mochi,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        m: usize,
        b: [usize; m],
        x: usize,
    }

    let mut dp = vec![State::Open; x + 1];
    for b in b {
        dp[b] = State::Mochi;
    }
    dp[0] = State::Possible;
    for i in 1..=x {
        if dp[i] == State::Mochi {
            continue;
        }
        for &d in &a {
            if let Some(j) = i.checked_sub(d) {
                if dp[j] == State::Possible {
                    dp[i] = State::Possible;
                    continue;
                }
            }
        }
    }

    println!(
        "{}",
        if dp[x] == State::Possible {
            "Yes"
        } else {
            "No"
        }
    );
}

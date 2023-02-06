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

#[derive(Debug, Default)]
struct Trie {
    popl: usize,
    child: [Option<Box<Trie>>; 26],
}

impl Trie {
    fn insert(&mut self, s: &mut impl Iterator<Item = char>) {
        self.popl += 1;
        if let Some(c) = s.next() {
            let c = c as usize - 'a' as usize;
            if self.child[c].is_none() {
                self.child[c] = Some(Box::default());
            }
            self.child[c].as_mut().unwrap().insert(s);
        }
    }

    fn max_lcp(&self, s: &mut impl Iterator<Item = char>) -> usize {
        if self.popl <= 1 {
            return 0;
        }
        if let Some(c) = s.next() {
            let c = c as usize - 'a' as usize;
            self.child[c].as_ref().unwrap().max_lcp(s) + 1
        } else {
            1
        }
    }
}

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }
    let mut trie = Trie::default();
    for s in &s {
        trie.insert(&mut s.chars());
    }
    for s in &s {
        println!("{}", trie.max_lcp(&mut s.chars()) - 1);
    }
}

// Input macro from https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
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

use std::cell::Cell;

#[derive(Debug, Copy, Clone)]
enum VertexType {
    Src,
    Sink,
    MToG,
    GToM,
}

fn solve(r: usize, c: usize, start: &[Vec<char>], end: &[Vec<char>], f: i64, s: i64) -> i64 {
    assert_eq!((f, s), (1, 1));
    let mut vs = vec![(0, 0, VertexType::Src), (0, 0, VertexType::Sink)];
    let mut diff = 0;
    for i in 0..r {
        for j in 0..c {
            match (start[i][j], end[i][j]) {
                ('M', 'G') => {
                    vs.push((i as isize, j as isize, VertexType::MToG));
                    diff += 1;
                }
                ('G', 'M') => {
                    vs.push((i as isize, j as isize, VertexType::GToM));
                    diff += 1;
                }
                _ => (),
            }
        }
    }
    let mut es = vec![vec![]; vs.len()];
    for i in 0..vs.len() {
        for j in 0..vs.len() {
            match (vs[i].2, vs[j].2) {
                (VertexType::Src, VertexType::MToG) | (VertexType::GToM, VertexType::Sink) => {
                    add_edge(i, j, 1, &mut es);
                }
                (VertexType::MToG, VertexType::GToM) => {
                    if (vs[i].0 - vs[j].0).abs() + (vs[i].1 - vs[j].1).abs() == 1 {
                        add_edge(i, j, 1, &mut es);
                    }
                }
                _ => (),
            }
        }
    }
    let flow = max_flow(0, 1, &es);

    diff - flow
}

#[derive(Debug, Clone)]
struct Edge {
    pub to: usize,
    pub cap: Cell<i64>,
    pub rev: usize,
}

fn add_edge(from: usize, to: usize, cap: i64, es: &mut Vec<Vec<Edge>>) {
    let from_len = es[from].len();
    let to_len = es[to].len();
    es[from].push(Edge {
        to,
        cap: Cell::new(cap),
        rev: to_len,
    });
    es[to].push(Edge {
        to: from,
        cap: Cell::new(0),
        rev: from_len,
    });
}

fn dfs(v: usize, t: usize, f: i64, es: &Vec<Vec<Edge>>, used: &mut Vec<bool>) -> i64 {
    if v == t {
        return f;
    }
    used[v] = true;
    for e in &es[v] {
        let cap = e.cap.get();
        if !used[e.to] && cap > 0 {
            let d = dfs(e.to, t, f.min(cap), es, used);
            if d > 0 {
                e.cap.set(cap - d);
                let r_cap = es[e.to][e.rev].cap.get();
                es[e.to][e.rev].cap.set(r_cap + d);
                return d;
            }
        }
    }
    0
}

fn max_flow(s: usize, t: usize, es: &Vec<Vec<Edge>>) -> i64 {
    let mut flow = 0;
    loop {
        let mut used = vec![false; es.len()];
        let f = dfs(s, t, 1 << 60, es, &mut used);
        if f == 0 {
            break flow;
        }
        flow += f;
    }
}

fn main() {
    let s = {
        use std::io::Read;
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    };
    let mut iter = s.split_whitespace();
    input_inner! {
        iter,
        t: usize,
    }
    for cs in 1..=t {
        input_inner! {
            iter,
            r: usize,
            c: usize,
            f: i64,
            s: i64,
            start: [chars; r],
            end: [chars; r],
        }
        println!("Case #{}: {}", cs, solve(r, c, &start, &end, f, s));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

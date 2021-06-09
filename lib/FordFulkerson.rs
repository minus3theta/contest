use std::cell::Cell;

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

use cargo_snippet::snippet;

#[snippet(prefix = "use max_flow::*;")]
#[snippet(prefix = "use MaxFlow;")]
pub mod max_flow {
    use std::cell::Cell;

    #[derive(Debug, Clone)]
    pub struct Edge<E> {
        pub to: usize,
        pub cap: Cell<i64>,
        pub weight: E,
        pub rev: usize,
    }

    #[derive(Debug, Clone)]
    pub struct FlowGraph<E> {
        pub edges: Vec<Vec<Edge<E>>>,
    }

    #[allow(clippy::len_without_is_empty)]
    impl<E> FlowGraph<E> {
        pub fn new(n: usize) -> Self {
            Self {
                edges: (0..n).map(|_| Vec::new()).collect(),
            }
        }
        pub fn len(&self) -> usize {
            self.edges.len()
        }
    }

    impl<E: Clone> FlowGraph<E> {
        pub fn add_edge(&mut self, from: usize, to: usize, cap: i64, weight: E) {
            let from_len = self.edges[from].len();
            let to_len = self.edges[to].len();
            self.edges[from].push(Edge {
                to,
                cap: Cell::new(cap),
                weight: weight.clone(),
                rev: to_len,
            });
            self.edges[to].push(Edge {
                to: from,
                cap: Cell::new(0),
                weight,
                rev: from_len,
            });
        }
    }

    pub trait MaxFlow<E> {
        fn max_flow(graph: &FlowGraph<E>, s: usize, t: usize) -> i64;
    }

    #[derive(Debug, Clone)]
    pub struct BipartiteMatching<E> {
        pub graph: FlowGraph<E>,
        pub a: usize,
        pub b: usize,
        pub source: usize,
        pub sink: usize,
    }

    impl<E: Clone + Default> BipartiteMatching<E> {
        pub fn new(a: usize, b: usize) -> Self {
            let mut graph = FlowGraph::new(a + b + 2);
            let source = a + b;
            let sink = a + b + 1;
            for i in 0..a {
                graph.add_edge(source, i, 1, Default::default());
            }
            for j in 0..b {
                graph.add_edge(a + j, sink, 1, Default::default());
            }
            Self {
                graph,
                a,
                b,
                source,
                sink,
            }
        }
        pub fn add_edge(&mut self, u: usize, v: usize, weight: E) {
            self.graph.add_edge(u, self.a + v, 1, weight);
        }
        pub fn max_match<F: MaxFlow<E>>(&self) -> Vec<&Edge<E>> {
            F::max_flow(&self.graph, self.source, self.sink);
            let mut ret = vec![];
            for i in 0..self.a {
                for e in &self.graph.edges[i] {
                    if e.to < self.source && e.cap.get() == 0 {
                        ret.push(e);
                    }
                }
            }
            ret
        }
    }
}

#[snippet(prefix = "use ford_fulkerson::*;")]
#[snippet(include = "max_flow")]
pub mod ford_fulkerson {
    use super::max_flow::{FlowGraph, MaxFlow};

    pub enum FordFulkerson {}

    impl<E> MaxFlow<E> for FordFulkerson {
        fn max_flow(graph: &FlowGraph<E>, s: usize, t: usize) -> i64 {
            let mut flow = 0;
            loop {
                let mut used = vec![false; graph.edges.len()];
                let f = Self::dfs(graph, s, t, std::i64::MAX, &mut used);
                if f == 0 {
                    break flow;
                }
                flow += f;
            }
        }
    }

    impl FordFulkerson {
        fn dfs<E>(graph: &FlowGraph<E>, v: usize, t: usize, f: i64, used: &mut Vec<bool>) -> i64 {
            if v == t {
                return f;
            }
            used[v] = true;
            for e in &graph.edges[v] {
                let cap = e.cap.get();
                if !used[e.to] && cap > 0 {
                    let d = Self::dfs(graph, e.to, t, f.min(cap), used);
                    if d > 0 {
                        e.cap.set(cap - d);
                        let r_cap = graph.edges[e.to][e.rev].cap.get();
                        graph.edges[e.to][e.rev].cap.set(r_cap + d);
                        return d;
                    }
                }
            }
            0
        }
    }
}

#[snippet(prefix = "use dinic::*;")]
#[snippet(include = "max_flow")]
pub mod dinic {
    use super::max_flow::{FlowGraph, MaxFlow};

    pub enum Dinic {}

    impl<E> MaxFlow<E> for Dinic {
        fn max_flow(graph: &FlowGraph<E>, s: usize, t: usize) -> i64 {
            let mut flow = 0;
            loop {
                let level = Self::bfs(graph, s);
                if level[t].is_none() {
                    return flow;
                }
                let mut current_edge = vec![0; graph.len()];
                loop {
                    let f = Self::dfs(graph, s, t, std::i64::MAX, &level, &mut current_edge);
                    if f == 0 {
                        break;
                    }
                    flow += f;
                }
            }
        }
    }

    impl Dinic {
        /// Calculate distance from s
        fn bfs<E>(graph: &FlowGraph<E>, s: usize) -> Vec<Option<i32>> {
            let mut level = vec![None; graph.len()];
            level[s] = Some(0);
            let mut que = std::collections::VecDeque::new();
            que.push_back(s);
            while let Some(v) = que.pop_front() {
                for e in &graph.edges[v] {
                    if e.cap.get() > 0 && level[e.to].is_none() {
                        level[e.to] = Some(level[v].unwrap() + 1);
                        que.push_back(e.to);
                    }
                }
            }
            level
        }

        fn dfs<E>(
            graph: &FlowGraph<E>,
            v: usize,
            t: usize,
            flow: i64,
            level: &[Option<i32>],
            current_edge: &mut [usize],
        ) -> i64 {
            if v == t {
                return flow;
            }
            while current_edge[v] < graph.edges[v].len() {
                let e = &graph.edges[v][current_edge[v]];
                if let Some(l) = level[v] {
                    let cap = e.cap.get();
                    if cap > 0 && level[e.to] == Some(l + 1) {
                        let d = Self::dfs(graph, e.to, t, flow.min(cap), level, current_edge);
                        if d > 0 {
                            e.cap.set(cap - d);
                            let r_cap = graph.edges[e.to][e.rev].cap.get();
                            graph.edges[e.to][e.rev].cap.set(r_cap + d);
                            return d;
                        }
                    }
                }
                current_edge[v] += 1;
            }
            0
        }
    }
}

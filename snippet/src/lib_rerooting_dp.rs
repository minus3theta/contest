use cargo_snippet::snippet;

#[snippet(prefix = "use rerooting_dp::*;")]
pub mod rerooting_dp {
    use std::fmt::Debug;

    pub struct RerootingDP<V, E, M, FL, FM> {
        vertex: Vec<V>,
        child: Vec<Vec<(usize, E)>>,
        unit: M,
        lift: FL,
        mul: FM,
    }

    impl<V, E, M, FL, FM> RerootingDP<V, E, M, FL, FM>
    where
        E: Clone,
        M: Clone,
        FL: Fn(&M, &V, &E) -> M + Copy,
        FM: Fn(&M, &M) -> M + Copy,
    {
        pub fn new(
            vertex: Vec<V>,
            child: Vec<Vec<(usize, E)>>,
            unit: M,
            lift: FL,
            mul: FM,
        ) -> Self {
            Self {
                vertex,
                child,
                unit,
                lift,
                mul,
            }
        }

        pub fn from_edges(
            vertex: Vec<V>,
            edges: &mut impl Iterator<Item = (usize, usize, E)>,
            unit: M,
            lift: FL,
            mul: FM,
        ) -> Self {
            let mut adj = vec![vec![]; vertex.len()];
            for (u, v, e) in edges {
                adj[u].push((v, e.clone()));
                adj[v].push((u, e));
            }
            let mut child = vec![vec![]; vertex.len()];
            Self::build_tree(&adj, &mut child, 0, None);
            Self {
                vertex,
                child,
                unit,
                lift,
                mul,
            }
        }

        fn build_tree(
            adj: &[Vec<(usize, E)>],
            child: &mut [Vec<(usize, E)>],
            v: usize,
            prev: Option<usize>,
        ) {
            for &(u, ref e) in &adj[v] {
                if prev == Some(u) {
                    continue;
                }
                child[v].push((u, e.clone()));
                Self::build_tree(adj, child, u, Some(v));
            }
        }

        pub fn solve(&self) -> Vec<M> {
            let mut m_sub = vec![self.unit.clone(); self.vertex.len()];
            self.bottom_up(0, &mut m_sub);
            let mut m_all = vec![self.unit.clone(); self.vertex.len()];
            self.top_down(0, self.unit.clone(), &m_sub, &mut m_all);
            m_all
        }

        fn bottom_up(&self, v: usize, m_sub: &mut [M]) -> M {
            let mut m = self.unit.clone();
            for &(c, ref e) in &self.child[v] {
                let mc = (self.lift)(&self.bottom_up(c, m_sub), &self.vertex[c], e);
                m = (self.mul)(&m, &mc);
            }
            m_sub[v] = m.clone();
            m
        }

        fn top_down(&self, v: usize, down: M, m_sub: &[M], m_all: &mut [M]) {
            m_all[v] = (self.mul)(&m_sub[v], &down);
            let n = self.child[v].len();
            let f = |s: &mut M, &(c, ref e)| {
                *s = (self.mul)(s, &(self.lift)(&m_sub[c], &self.vertex[c], e));
                Some(s.clone())
            };
            let left_scan: Vec<M> = Some(self.unit.clone())
                .into_iter()
                .chain(self.child[v].iter().scan(self.unit.clone(), f))
                .take(n)
                .collect();
            let right_scan: Vec<M> = Some(self.unit.clone())
                .into_iter()
                .chain(self.child[v].iter().rev().scan(self.unit.clone(), f))
                .take(n)
                .collect();
            for ((c, e), (ref l, ref r)) in self.child[v]
                .iter()
                .zip(left_scan.into_iter().zip(right_scan.into_iter().rev()))
            {
                let d = (self.lift)(&(self.mul)(&down, &(self.mul)(&l, &r)), &self.vertex[v], e);
                self.top_down(*c, d, m_sub, m_all)
            }
        }
    }

    impl<V: Debug, E: Debug, M, FL, FM> Debug for RerootingDP<V, E, M, FL, FM> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.debug_struct("RerootingDP")
                .field("vertex", &self.vertex)
                .field("child", &self.child)
                .finish()
        }
    }
}

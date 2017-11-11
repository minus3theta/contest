template<class T = int>
class Dinic {
  struct edge {
    int to;
    T cap;
    int rev;
  };
  vector<vector<edge>> graph;
  vector<int> level;
  vector<int> iter;

  void bfs(int s) {
    level.assign(level.size(), -1);
    queue<int> que;
    level[s] = 0;
    que.push(s);
    while(!que.empty()) {
      int v = que.front();
      que.pop();
      for(const edge &e: graph[v]) {
        if(e.cap > 0 && level[e.to] < 0) {
          level[e.to] = level[v] + 1;
          que.push(e.to);
        }
      }
    }
  }

  T dfs(int v, int t, T f) {
    if(v == t) return f;
    for(int &i = iter[v]; i < (int)graph[v].size(); i++) {
      edge &e = graph[v][i];
      if(e.cap > 0 && level[v] < level[e.to]) {
        T new_f = f < 0 ? e.cap : min(f, e.cap);
        int d = dfs(e.to, t, new_f);
        if(d > 0) {
          e.cap -= d;
          graph[e.to][e.rev].cap += d;
          return d;
        }
      }
    }
    return 0;
  }

public:
  Dinic(int n) : graph(n), level(n), iter(n) {}

  void add_edge(int from, int to, T cap) {
    graph[from].push_back({to, cap, (int)graph[to].size()});
    graph[to].push_back({from, 0, (int)graph[from].size() - 1});
  }

  T max_flow(int s, int t) {
    T flow = 0;
    while(true) {
      bfs(s);
      if(level[t] < 0) return flow;
      iter.assign(iter.size(), 0);
      T f;
      while((f = dfs(s, t, -1)) > 0) {
        flow += f;
      }
    }
  }
};

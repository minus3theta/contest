#include <cstdio>
#include <climits>
#include <cassert>
#include <cmath>
#include <iostream>
#include <string>
#include <algorithm>
#include <numeric>
#include <functional>
#include <tuple>
#include <list>
#include <map>
#include <queue>
#include <stack>
#include <set>
#include <vector>
#include <iterator>
#include <regex>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

template <class T, template <class, class> class C, class charT = char>
void vdump(const C<T, allocator<T>> &v, const charT* delimiter = ", ",
           ostream &stream = cout) {
  copy(v.begin(), v.end(), ostream_iterator<T>(stream, delimiter));
  stream << endl;
}

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

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  vector<PI> red(n);
  vector<PI> blue(n);
  FOR(r,red) {
    cin >> r.first >> r.second;
  }
  FOR(b,blue) {
    cin >> b.first >> b.second;
  }
  Dinic<int> g(2 * n + 2);
  REP(i,0,n) {
    g.add_edge(2 * n, i, 1);
  }
  REP(i,n,2*n) {
    g.add_edge(i, 2 * n + 1, 1);
  }
  REP(r,0,n) {
    REP(b,0,n) {
      if(red[r].first < blue[b].first && red[r].second < blue[b].second) {
        g.add_edge(r, b + n, 1);
      }
    }
  }
  cout << g.max_flow(2 * n, 2 * n + 1) << endl;
  
  return 0;
}

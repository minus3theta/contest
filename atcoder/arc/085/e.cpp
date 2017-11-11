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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

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

const ll INF = 1L << 60;

int main() {
  ios::sync_with_stdio(false);
  int N;
  cin >> N;
  Dinic<ll> din(N + 2);
  ll pos = 0;
  REP(i,0,N) {
    ll a, b;
    cin >> a;
    if(a >= 0) {
      pos += a;
      b = 0;
    } else {
      b = -a;
      a = 0;
    }
    din.add_edge(0, i+1, b);
    din.add_edge(i+1, N+1, a);
  }
  REP(i,1,N+1) {
    for(int j=i*2; j<=N; j+=i) {
      din.add_edge(i, j, INF);
    }
  }
  ll ans = pos - din.max_flow(0, N+1);
  cout << ans << endl;
  
  return 0;
}

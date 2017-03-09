#include <cstdio>
#include <climits>
#include <cassert>
#include <cmath>
#include <iostream>
#include <string>
#include <algorithm>
#include <numeric>
#include <functional>
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

const int LEVEL = 3;

struct edge {
  int to, cap, rev;
  bool is_rev;
};

class graph {
private:
  int n;
  vector<vector<edge> > es;
  vector<bool> used;
public:
  graph(int n) : n(n), es(n), used(n, false) {}
  void add_edge(int src, int dst, int cap) {
    es[src].push_back((edge){dst, cap, (int)es[dst].size(), false});
    es[dst].push_back((edge){src, 0, (int)es[src].size() - 1, true});
  }
  int dfs(int v, int t, int f) {
    if(v == t) return f;
    used[v] = true;
    REP(i,0,es[v].size()) {
      edge &e = es[v][i];
      if(!used[e.to] && e.cap > 0) {
        int d = dfs(e.to, t, min(f, e.cap));
        if(d > 0) {
          e.cap -= d;
          es[e.to][e.rev].cap += d;
          return d;
        }
      }
    }
    return 0;
  }
  int max_flow(int s, int t) {
    int flow = 0;
    while(true) {
      REP(i,0,n) {
        used[i] = false;
      }
      int f = dfs(s, t, INT_MAX);
      if(f == 0) {
        // cout << s << " -> " << t << " : " << flow << endl;
        return flow;
      }
      flow += f;
      if(flow >= LEVEL) {
        return flow;
      }
    }
  }
  void reset() {
    REP(i,0,n) {
      REP(j,0,es[i].size()) {
        es[i][j].cap = es[i][j].is_rev ? 0 : 1;
      }
    }
  }
};

int main() {
  // ios::sync_with_stdio(false);
  int N, M;
  while(true) {
    assert(scanf("%d %d", &N, &M) == 2);
    if(!N && !M) break;
    graph g(N * 2);
    REP(i,0,N) {
      g.add_edge(i * 2, i * 2 + 1, 1);
    }
    REP(i,0,M) {
      int s, t;
      assert(scanf("%d %d", &s, &t) == 2);
      g.add_edge(s * 2 + 1, t * 2, 1);
      g.add_edge(t * 2 + 1, s * 2, 1);
    }
    bool ans = true;
    REP(i,0,N) {
      if(!ans) break;
      REP(j,0,i) {
        g.reset();
        if(g.max_flow(i * 2 + 1, j * 2) < LEVEL) {
          ans = false;
          break;
        }
      }
    }
    printf(ans ? "YES\n" : "NO\n");
  }
  return 0;
}

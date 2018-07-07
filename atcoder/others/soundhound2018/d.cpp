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

struct edge {
  int u, v;
  ll cost[2];
};

struct vtx {
  ll dist[2] = {(ll)1e18, (ll)1e18};
  vector<edge> es;
};

void dijk(int key, vector<vtx> &vs, int start) {
  priority_queue<pair<ll,int>, vector<pair<ll,int>>, greater<pair<ll,int>>> que;
  vs[start].dist[key] = 0;
  que.push({0, start});
  while(!que.empty()) {
    auto p = que.top();
    ll d = p.first;
    vtx &u = vs[p.second];
    que.pop();
    FOR(e, u.es) {
      vtx &v = vs[e.v];
      if(d + e.cost[key] < v.dist[key]) {
        v.dist[key] = d + e.cost[key];
        que.push({v.dist[key], e.v});
      }
    }
  }
}

constexpr ll account = 1e15;

int main() {
  ios::sync_with_stdio(false);
  int n, m, s, t;
  cin >> n >> m >> s >> t;
  vector<vtx> vs(n+1);
  REP(i,0,m) {
    int u, v;
    ll a, b;
    cin >> u >> v >> a >> b;
    vs[u].es.push_back({u, v, {a, b}});
    vs[v].es.push_back({v, u, {a, b}});
  }
  dijk(0, vs, s);
  dijk(1, vs, t);
  VL cost(n);
  cost[n-1] = vs[n].dist[0] + vs[n].dist[1];
  for(int i=n-2; i>=0; i--) {
    cost[i] = min(cost[i+1], vs[i+1].dist[0] + vs[i+1].dist[1]);
  }
  FOR(c,cost) {
    cout << account - c << endl;
  }
  
  return 0;
}

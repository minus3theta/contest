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

const ll mod = 1e9+7;
const ll inf = 1e18;

struct edge {
  int to;
  ll d;
};

struct vtx {
  vector<edge> es;
  ll dist[2] = {inf, inf};
  ll pattern[2] = {0, 0};
};

void dijk(vector<vtx> &vs, int s, int dir) {
  vs[s].dist[dir] = 0;
  priority_queue<pair<ll, int>, vector<pair<ll, int>>, greater<>> que;
  que.push({0, s});
  while(!que.empty()) {
    pair<ll, int> t = que.top();
    que.pop();
    ll dist = t.first;
    int x = t.second;
    if(dist > vs[x].dist[dir]) continue;
    FOR(e,vs[x].es) {
      if(dist + e.d < vs[e.to].dist[dir]) {
        vs[e.to].dist[dir] = dist + e.d;
        que.push({vs[e.to].dist[dir], e.to});
      }
    }
  }
}

void count(vector<vtx> &vs, int s, int dir) {
  int n = vs.size() - 1;
  vs[s].pattern[dir] = 1;
  vector<pair<ll, int>> v_sort;
  REP(i,1,n+1) {
    v_sort.push_back({vs[i].dist[dir], i});
  }
  sort(v_sort.begin(), v_sort.end());
  FOR(p,v_sort) {
    int x = p.second;
    FOR(e, vs[x].es) {
      if(vs[x].dist[dir] + e.d == vs[e.to].dist[dir]) {
        vs[e.to].pattern[dir] += vs[x].pattern[dir];
        vs[e.to].pattern[dir] %= mod;
      }
    }
  }
}

ll collision_point(vector<vtx> &vs, ll diam) {
  if(diam % 2) return 0;
  int n = vs.size() - 1;
  ll pat = 0;
  REP(i,1,n+1) {
    if(vs[i].dist[0] == diam / 2 &&
       vs[i].dist[1] == diam / 2) {
      ll rt = vs[i].pattern[0] * vs[i].pattern[1] % mod;
      pat += rt * rt % mod;
      pat %= mod;
    }
  }
  return pat;
}

ll collision_edge(vector<vtx> &vs, ll diam) {
  int n = vs.size() - 1;
  ll pat = 0;
  REP(i,1,n+1) {
    if(vs[i].dist[0] + vs[i].dist[1] != diam) continue;
    FOR(e, vs[i].es) {
      if(vs[i].dist[0] + e.d == vs[e.to].dist[0] && 
         vs[e.to].dist[1] + e.d == vs[i].dist[1]) {
        if(2 * vs[i].dist[0] < diam && diam < 2 * vs[e.to].dist[0]) {
          ll rt = vs[i].pattern[0] * vs[e.to].pattern[1] % mod;
          pat += rt * rt % mod;
          pat %= mod;
        }
      }
    }
  }
  return pat;
}

int main() {
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  int s, t;
  cin >> s >> t;
  vector<vtx> vs(n+1);
  REP(i,0,m) {
    int u, v;
    ll d;
    cin >> u >> v >> d;
    vs[u].es.push_back({v, d});
    vs[v].es.push_back({u, d});
  }
  dijk(vs, s, 0);
  dijk(vs, t, 1);
  count(vs, s, 0);
  count(vs, t, 1);
  ll diam = vs[t].dist[0];
  ll total_one = vs[t].pattern[0];
  ll ans = (total_one * total_one) % mod;
  ans = (ans - collision_point(vs, diam) + mod) % mod;
  ans = (ans - collision_edge(vs, diam) + mod) % mod;
  cout << ans << endl;

  return 0;
}

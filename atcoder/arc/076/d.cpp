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

struct town {
  ll x;
  ll y;
  int id;
};

struct edge {
  ll cost;
  int src;
  int dst;
};

struct uf {
  int n;
  VI p;
  uf(int n) : n(n), p(n) {
    REP(i,0,n) {
      p[i] = i;
    }
  }
  int root(int x) {
    if(p[x] == x) return x;
    return p[x] = root(p[x]);
  }
  void unite(int x, int y) {
    p[root(x)] = root(y);
  }
  bool find(int x, int y) {
    return root(x) == root(y);
  }
};

int main() {
  ios::sync_with_stdio(false);
  int N;
  cin >> N;
  vector<town> ts(N);
  FOR(t,ts) {
    cin >> t.x >> t.y;
  }
  REP(i,0,N) {
    ts[i].id = i;
  }
  vector<edge> es;
  sort(ts.begin(), ts.end(), [](town a, town b){return a.x < b.x;});
  REP(i,0,N-1) {
    town a = ts[i];
    town b = ts[i+1];
    es.push_back({b.x-a.x, a.id, b.id});
  }
  sort(ts.begin(), ts.end(), [](town a, town b){return a.y < b.y;});
  REP(i,0,N-1) {
    town a = ts[i];
    town b = ts[i+1];
    es.push_back({b.y-a.y, a.id, b.id});
  }
  sort(es.begin(), es.end(), [](edge a, edge b){return a.cost < b.cost;});
  uf u(N);
  ll ans = 0;
  FOR(e,es) {
    if(u.find(e.src, e.dst)) continue;
    ans += e.cost;
    u.unite(e.src, e.dst);
  }
  cout << ans << endl;
  return 0;
}

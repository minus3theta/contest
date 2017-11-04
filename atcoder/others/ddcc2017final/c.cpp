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

struct edge {
  int a, b;
  ll c;
  bool inv;
};

struct vtx {
  ll p;
  vector<edge> es;
};

void dfs_tree(int a, vector<vtx> &vs, vector<edge> &tree) {
  vtx &va = vs[a];
  FOR(e, va.es) {
    vtx &vb = vs[e.b];
    if(vb.p == LLONG_MAX) {
      vb.p = va.p + e.c;
      if(e.inv) {
        tree.push_back({e.b, e.a, -e.c, false});
      } else {
        tree.push_back(e);
      }
      dfs_tree(e.b, vs, tree);
    }
  }
}

void add_contradiction(const vector<vtx> &vs, vector<edge> &tree) {
  FOR(va, vs) {
    FOR(e, va.es) {
      if(!e.inv && vs[e.b].p != va.p + e.c) {
        tree.push_back(e);
        return;
      }
    }
  }
}

bool dfs(int a, const edge &e0, vector<vtx> &vs) {
  vtx &va = vs[a];
  FOR(e, va.es) {
    if(e.inv || (e.a == e0.a && e.b == e0.b)) continue;
    vtx &vb = vs[e.b];
    if(vb.p == LLONG_MAX) {
      vb.p = va.p + e.c;
      if(!dfs(e.b, e0, vs)) {
        return false;
      }
    }
    if(vb.p != va.p + e.c) {
      return false;
    }
  }
  return true;
}

bool test(const edge &e, vector<vtx> &vs) {
  FOR(v,vs) {
    v.p = LLONG_MAX;
  }
  vs[e.b].p = 0;
  if(dfs(e.b, e, vs)) {
    return true;
  } else {
    return false;
  }
}


int main() {
  ios::sync_with_stdio(false);
  int N, M;
  cin >> N >> M;
  vector<vtx> vs(N, {LLONG_MAX, vector<edge>()});
  vector<edge> es;
  REP(i,0,M) {
    int a, b;
    ll c;
    cin >> a >> b >> c;
    a--;
    b--;
    vs[a].es.push_back({a, b, c, false});
    vs[b].es.push_back({b, a, -c, true});
    es.push_back({a, b, c, false});
  }
  vector<edge> tree;
  dfs_tree(0, vs, tree);
  add_contradiction(vs, tree);
  bool ans = any_of(tree.begin(), tree.end(), [&](const edge &e){return test(e, vs);});
  cout << (ans ? "Yes" : "No") << endl;
  
  return 0;
}

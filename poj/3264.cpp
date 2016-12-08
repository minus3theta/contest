#include <cstdio>
#include <climits>
#include <cassert>
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

template <class T, T def, class Select>
struct segtree {
  int n;
  vector<T> dat;
  Select sel;
  segtree(int al, Select sel = Select()) {
    n = 1;
    while(n < al) {
      n *= 2;
    }
    dat = vector<T>(2 * n - 1, def);
    this->sel = sel;
  }
  void update(int k, T a) {
    k += n - 1;
    dat[k] = a;
    while(k > 0) {
      k = (k - 1) / 2;
      dat[k] = sel(dat[k * 2 + 1], dat[k * 2 + 2]);
    }
  }
  T query(int a, int b, int k = 0, int l = 0, int r = -1) {
    if(r < 0) r = n;
    if(r <= a || b <= l) return def;
    if(a <= l && r <= b) return dat[k];
    T vl = query(a, b, k * 2 + 1, l, (l + r) / 2);
    T vr = query(a, b, k * 2 + 2, (l + r) / 2, r);
    return sel(vl, vr);
  }
};

struct lmin {
  ll operator()(const ll &a, const ll &b) {
    return min(a, b);
  }
};

struct lmax {
  ll operator()(const ll &a, const ll &b) {
    return max(a, b);
  }
};

int main() {
  // ios::sync_with_stdio(false);
  int N, Q;
  assert(scanf("%d %d", &N, &Q) == 2);
  VL h(N);
  REP(i,0,N) {
    assert(scanf("%lld", &h[i]) == 1);
  }
  vector<PI> qry(Q);
  REP(i,0,Q) {
    assert(scanf("%d %d", &qry[i].first, &qry[i].second) == 2);
  }
  segtree<ll, 1LL << 62, lmin> mintree(N);
  segtree<ll, -(1LL << 62), lmax> maxtree(N);
  REP(i,0,N) {
    mintree.update(i, h[i]);
    maxtree.update(i, h[i]);
  }
  REP(i,0,Q) {
    ll lb = mintree.query(qry[i].first - 1, qry[i].second);
    ll ub = maxtree.query(qry[i].first - 1, qry[i].second);
    printf("%lld\n", ub - lb);
  }
  return 0;
}

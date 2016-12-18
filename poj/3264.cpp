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

template <class T, class Op, T unit>
struct Segtree {
  int n;
  vector<T> dat;
  Op op;
  Segtree(int al) {
    n = 1;
    while(n < al) {
      n *= 2;
    }
    dat = vector<T>(2 * n - 1, unit);
  }
  Segtree(const vector<T> &arr) {
    int al = arr.size();
    n = 1;
    while(n < al) {
      n *= 2;
    }
    dat = vector<T>(2 * n - 1, unit);
    copy(arr.begin(), arr.end(), dat.begin() + n - 1);
    for(int i=n-2; i >= 0; i--) {
      dat[i] = op(dat[i * 2 + 1], dat[i * 2 + 2]);
    }
  }
  void update(int k, T a) {
    k += n - 1;
    dat[k] = a;
    while(k > 0) {
      k = (k - 1) / 2;
      dat[k] = op(dat[k * 2 + 1], dat[k * 2 + 2]);
    }
  }
  T query(int a, int b, int k = 0, int l = 0, int r = -1) {
    if(r < 0) r = n;
    if(r <= a || b <= l) return unit;
    if(a <= l && r <= b) return dat[k];
    T vl = query(a, b, k * 2 + 1, l, (l + r) / 2);
    T vr = query(a, b, k * 2 + 2, (l + r) / 2, r);
    return op(vl, vr);
  }
};

struct lmin {
  ll operator()(ll a, ll b) {
    return min(a, b);
  }
};

struct lmax {
  ll operator()(ll a, ll b) {
    return max(a, b);
  }
};

int main() {
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
  Segtree<ll, lmin, 1LL << 62> mintree(h);
  Segtree<ll, lmax, -(1LL << 62)> maxtree(h);
  REP(i,0,Q) {
    ll lb = mintree.query(qry[i].first - 1, qry[i].second);
    ll ub = maxtree.query(qry[i].first - 1, qry[i].second);
    printf("%lld\n", ub - lb);
  }
  return 0;
}

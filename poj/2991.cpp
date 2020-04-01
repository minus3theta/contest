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
#include <iterator>
#include <complex>
#include <iomanip>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

// 0-indexed
template <class T, class Op = T (*) (T, T)>
struct Segtree {
  int n;
  vector<T> dat;
  Op op;
  T unit;
  Segtree(int al, Op op = Op(), T unit = T()): op(op), unit(unit) {
    n = 1;
    while(n < al) {
      n *= 2;
    }
    dat = vector<T>(2 * n - 1, unit);
  }
  Segtree(const vector<T> &arr, Op op = Op(), T unit = T()): op(op), unit(unit) {
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
  void update(int k, T x) {
    k += n - 1;
    dat[k] = x;
    while(k > 0) {
      k = (k - 1) / 2;
      dat[k] = op(dat[k * 2 + 1], dat[k * 2 + 2]);
    }
  }
  // accumlate [a, b)
  T accum(int a, int b, int k = 0, int l = 0, int r = -1) {
    if(r < 0) r = n;
    if(r <= a || b <= l) return unit;
    if(a <= l && r <= b) return dat[k];
    T vl = accum(a, b, k * 2 + 1, l, (l + r) / 2);
    T vr = accum(a, b, k * 2 + 2, (l + r) / 2, r);
    return op(vl, vr);
  }
};

typedef pair<double, complex<double> > Elem;

struct Add {
  Elem operator()(Elem &a, Elem &b) {
    return make_pair(a.first + b.first, a.second + polar(1.0, a.first) * b.second);
  }
};

int main() {
  double MPI = acos(-1.0);
  int n, c;
  while (scanf("%d %d", &n, &c) == 2) {
    vector<Elem> arms;
    arms.push_back(make_pair(MPI / 2.0, complex<double>()));
    REP(i,0,n) {
      double l;
      scanf("%lf", &l);
      arms.push_back(make_pair(0.0, complex<double>(l, 0.0)));
    }
    Segtree<Elem, Add> t(arms, Add(), make_pair(0.0, complex<double>()));
    REP(i,0,c) {
      int s;
      double a;
      scanf("%d %lf", &s, &a);
      t.update(s, make_pair((a - 180.0) / 180.0 * MPI, arms[s].second));
      Elem top = t.accum(0, n+1);
      printf("%.15f %.15f\n", top.second.real(), top.second.imag());
    }
    printf("\n");
  }

  return 0;
}

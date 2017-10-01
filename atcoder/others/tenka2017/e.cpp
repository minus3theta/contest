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
#include <iomanip>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

// 1-indexed
template <class T, class Op, T unit>
struct Bit {
  int n;
  vector<T> dat;
  Op op;
  Bit(int n) : n(n), dat(n + 1, unit) {
  }
  Bit(const vector<T> &arr) : n(arr.size()), dat(arr.size() + 1) {
    copy(arr.begin(), arr.end(), dat.begin() + 1);
    for(int i=1; i<n; i++) {
      dat[i + (i & -i)] = op(dat[i + (i & -i)], dat[i]);
    }
  }
  void operate(int k, T a) {
    while(k <= n) {
      dat[k] = op(dat[k], a);
      k += k & -k;
    }
  }
  T accum(int k) {
    T sum = unit;
    while(k > 0) {
      sum = op(sum, dat[k]);
      k -= k & -k;
    }
    return sum;
  }
};

struct line {
  double A;
  double B;
  double C;
};

const int ITER = 70;

int inversion(const vector<pair<double, int>> &ps) {
  int N = ps.size();
  Bit<int, plus<int>, 0> bit(N);
  int inv = 0;
  FOR(p,ps) {
    inv += bit.accum(p.second + 1);
    bit.operate(p.second + 1, 1);
  }
  return inv;
}

double find_x(const vector<line> &ls, int idx) {
  int N = ls.size();
  double l = -2e8;
  double r = 2e8;
  vector<pair<double, int>> ps(N);
  REP(i,0,ITER) {
    double x = (l + r) / 2;
    REP(i,0,N) {
      const line &l = ls[i];
      ps[i] = {(l.C - l.A * x) / l.B, i};
    }
    sort(ps.rbegin(), ps.rend());
    int inv = inversion(ps);
    if(inv < idx) {
      l = x;
    } else {
      r = x;
    }
  }
  return l;
}

double find_y(const vector<line> &ls, int idx) {
  int N = ls.size();
  double l = -2e8;
  double r = 2e8;
  vector<pair<double, int>> ps(N);
  REP(i,0,ITER) {
    double y = (l + r) / 2;
    REP(i,0,N) {
      const line &l = ls[i];
      ps[i] = {(l.C - l.B * y) / l.A, i};
    }
    sort(ps.rbegin(), ps.rend());
    int inv = inversion(ps);
    if(inv < idx) {
      l = y;
    } else {
      r = y;
    }
  }
  return l;
}
  

int main() {
  ios::sync_with_stdio(false);
  int N;
  cin >> N;
  vector<line> ls(N);
  FOR(l,ls) {
    cin >> l.A >> l.B >> l.C;
  }
  int idx = (N * (N-1) / 2 + 1) / 2;
  sort(ls.begin(), ls.end(), [](const line &l1, const line &l2) {
      return l1.A / l1.B < l2.A / l2.B;
    });
  double x = find_x(ls, idx);
  sort(ls.begin(), ls.end(), [](const line &l1, const line &l2) {
      return l1.B / l1.A < l2.B / l2.A;
    });
  double y = find_y(ls, idx);
  cout << setprecision(15) << x << " " << y << endl;

  return 0;
}

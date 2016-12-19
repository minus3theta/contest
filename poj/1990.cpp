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

const int X = 20000;

int main() {
  ios::sync_with_stdio(false);
  int N;
  cin >> N;
  vector<PI> cows(N);
  REP(i,0,N) {
    cin >> cows[i].first >> cows[i].second;
  }
  sort(cows.begin(), cows.end());
  Bit<ll, plus<ll>, 0> t0(X);
  Bit<ll, plus<ll>, 0> t1(X);
  ll ans = 0;
  REP(i,0,N) {
    int th = cows[i].first;
    int x = cows[i].second;
    ll all1 = t1.accum(X);
    ll part1 = t1.accum(x);
    ll all0 = t0.accum(X);
    ll part0 = t0.accum(x);
    ll lsum = x * part0 - part1;
    ll rsum = (all1 - part1) - x * (all0 - part0);
    ans += (lsum + rsum) * th;
    t0.operate(x, 1);
    t1.operate(x, x);
  }
  cout << ans << endl;
  return 0;
}

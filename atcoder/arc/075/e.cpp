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

// 1-indexed
template <class T, class Op = T (*) (T, T)>
struct Bit {
  int n;
  vector<T> dat;
  Op op;
  T unit;
  Bit(int n, Op op = Op(), T unit = T()) : n(n), dat(n + 1, unit), op(op), unit(unit) {
  }
  Bit(const vector<T> &arr, Op op = Op(), T unit = T())
    : n(arr.size()), dat(arr.size() + 1), op(op), unit(unit) {
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

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  ll k;
  cin >> k;
  VL as(n);
  FOR(a,as) {
    cin >> a;
  }
  VL bs(n+1);
  bs[0] = 0;
  REP(i,0,n) {
    bs[i+1] = bs[i] + as[i] - k;
  }
  VL b_sort = bs;
  sort(b_sort.begin(), b_sort.end());
  Bit<ll, plus<ll>> bit(n+1, plus<ll>(), 0);
  ll ans = 0;
  FOR(b,bs) {
    int c = lower_bound(b_sort.begin(), b_sort.end(), b) - b_sort.begin() + 1;
    ans += bit.accum(c);
    bit.operate(c, 1);
  }
  cout << ans << endl;

  return 0;
}

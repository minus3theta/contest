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
  string s;
  cin >> s;
  int n = s.size();
  vector<deque<int>> app(26);
  REP(i,0,n) {
    char c = s[i];
    app[c-'a'].push_back(i);
  }
  int odd = 0;
  REP(i,0,26) {
    if(app[i].size() % 2 == 1) {
      odd++;
    }
  }
  if(odd > 1) {
    cout << -1 << endl;
    return 0;
  }
  VI ps(n);
  vector<PI> head;
  REP(i,0,26) {
    int f = app[i].size();
    REP(j,0,f/2) {
      head.push_back({app[i].front(), i});
      app[i].pop_front();
    }
    if(f % 2 == 1) {
      ps[app[i].front()] = n / 2;
      app[i].pop_front();
    }
  }
  sort(head.begin(), head.end());
  REP(i,0,head.size()) {
    ps[head[i].first] = i;
    int c = head[i].second;
    ps[app[c].back()] = n - i - 1;
    app[c].pop_back();
  }
  ll ans = 0;
  Bit<ll, plus<ll>> bit(n);
  for_each(ps.rbegin(), ps.rend(), [&](int p) {
      ans += bit.accum(p + 1);
      bit.operate(p + 1, 1);
    });
  cout << ans << endl;

  return 0;
}

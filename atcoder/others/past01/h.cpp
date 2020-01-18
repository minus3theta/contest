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

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VL card(n);
  FOR(c,card) {
    cin >> c;
  }
  ll even_min = 1e18;
  ll odd_min = 1e18;
  REP(i,0,n) {
    if (i % 2 == 0) {
      even_min = min(even_min, card[i]);
    } else {
      odd_min = min(odd_min, card[i]);
    }
  }
  VL sell(n);
  ll sell_even = 0;
  ll sell_odd = 0;
  auto sold = [&](int x) {
    ll r = sell[x];
    if (x % 2 == 0) {
      r += sell_even;
    } else {
      r += sell_odd;
    }
    return r;
  };
  int q;
  cin >> q;
  REP(i,0,q) {
    int s;
    cin >> s;
    if (s == 1) {
      int x;
      ll a;
      cin >> x >> a;
      x--;
      ll r = sold(x);
      if (r + a <= card[x]) {
        sell[x] += a;
      }
      if (x % 2 == 0) {
        even_min = min(even_min, card[x] - sold(x));
      } else {
        odd_min = min(odd_min, card[x] - sold(x));
      }
    } else if (s == 2) {
      ll a;
      cin >> a;
      if (a <= even_min) {
        sell_even += a;
        even_min -= a;
      }
    } else {
      ll a;
      cin >> a;
      if (a <= even_min && a <= odd_min) {
        sell_even += a;
        even_min -= a;
        sell_odd += a;
        odd_min -= a;
      }
    }
  }
  ll s = sell_odd * (n / 2) + sell_even * ((n + 1) / 2);
  FOR(x,sell) {
    s += x;
  }
  cout << s << endl;

  return 0;
}

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
  ll n;
  cin >> n;
  VL divisor;
  for(ll i=1; i * i <= n; i++) {
    if (n % i == 0) {
      divisor.push_back(i);
      if (i * i != n) {
        divisor.push_back(n / i);
      }
    }
  }
  ll ans = 0;
  FOR(d,divisor) {
    ll m = d - 1;
    ll k = n / d;
    if (m < 1) continue;
    if (k < 0 || m <= k) continue;
    ans += m;
  }
  cout << ans << endl;

  return 0;
}

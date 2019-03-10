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

ll solve(ll k, ll a, ll b, ll x) {
  if (k < 2) {
    return k + x;
  }
  if (b - a < 2) {
    return k + x;
  }
  if (x < a) {
    ll y = a - x;
    return solve(k - y, a, b, x + y);
  }
  ll z = k / 2;
  return solve(k - 2 * z, a, b, x + (b - a) * z);
}

int main() {
  ios::sync_with_stdio(false);
  ll k, a, b;
  cin >> k >> a >> b;
  cout << solve(k, a, b, 1) << endl;

  return 0;
}

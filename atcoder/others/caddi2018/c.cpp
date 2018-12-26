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

ll pow(ll x, ll y) {
  if(y == 0) {
    return 1;
  }
  if(y % 2) {
    return pow(x, y - 1) * x;
  }
  ll z = pow(x, y / 2);
  return z * z;
}

int main() {
  ios::sync_with_stdio(false);
  ll n, p;
  cin >> n >> p;
  if(n == 1) {
    cout << p << endl;
    return 0;
  }
  ll ans = 1;
  for(ll i = 2; i * i <= p; i++) {
    ll j = 0;
    for(; p % i == 0; j++) {
      p /= i;
    }
    ans *= pow(i, j / n);
  }
  cout << ans << endl;

  return 0;
}

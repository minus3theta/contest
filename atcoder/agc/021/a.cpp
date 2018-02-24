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

ll sum(ll x) {
  ll s = 0;
  while(x != 0) {
    s += x % 10;
    x /= 10;
  }
  return s;
}

int main() {
  ios::sync_with_stdio(false);
  ll n;
  cin >> n;
  ll x = 10;
  ll ans = sum(n);
  while(x <= n) {
    ll a = n / x * x;
    ans = max(ans, sum(a - 1));
    x *= 10;
  }
  cout << ans << endl;
  
  return 0;
}

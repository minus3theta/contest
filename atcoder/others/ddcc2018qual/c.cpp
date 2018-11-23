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

constexpr ll MOD = 1e9 + 7;

ll powmod(ll x, ll y) {
  if(y == 0) return 1;
  if(y % 2) return x * powmod(x, y - 1) % MOD;
  ll a = powmod(x, y / 2);
  return a * a % MOD;
}

int main() {
  ios::sync_with_stdio(false);
  ll n;
  cin >> n;
  ll ans = 0;
  REP(maxp,1,n+1) {
    ll pc = (powmod(maxp, 10) - powmod(maxp - 1, 10) + MOD) % MOD;
    ll maxq = n / maxp;
    ll qc = powmod(maxq, 10);
    ans = (ans + pc * qc % MOD) % MOD;
  }
  cout << ans << endl;

  return 0;
}

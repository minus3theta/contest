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

ll ext_gcd(ll a, ll b, ll &x, ll &y) {
  if(b == 0) {
    x = 1;
    y = 0;
    return a;
  }
  ll q = a / b;
  ll gcd = ext_gcd(b, a - q * b, x, y);
  ll z = x - q * y;
  x = y;
  y = z;
  return gcd;
}

constexpr ll MOD = 998244353;

int main() {
  ios::sync_with_stdio(false);
  int n;
  ll a, b, k;
  cin >> n >> a >> b >> k;
  VL fact(n+1);
  fact[0] = 1;
  REP(i,1,n+1) {
    fact[i] = (fact[i-1] * i) % MOD;
  }
  VL inv_fact(n+1);
  ll x, y;
  ext_gcd(fact[n], MOD, x, y);
  inv_fact[n] = (x + MOD) % MOD;
  for(int i=n-1; i>=0; i--) {
    inv_fact[i] = (inv_fact[i+1] * (i+1)) % MOD;
  }

  ll ans = 0;
  for(ll i=0; i<n+1; i++) {
    ll x = a * i;
    if(k - x < 0 || (k - x) % b != 0) {
      continue;
    }
    ll j = (k - x) / b;
    if(j > n) continue;
    ll ca = ((fact[n] * inv_fact[i]) % MOD * inv_fact[n-i]) % MOD;
    ll cb = ((fact[n] * inv_fact[j]) % MOD * inv_fact[n-j]) % MOD;
    ans = (ans + (ca * cb) % MOD) % MOD;
  }
  cout << ans << endl;

  return 0;
}

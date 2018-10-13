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

constexpr ll MOD = 1e9+7;
constexpr ll LIM = 1e5+10;

ll mod_pow(ll x, ll exp) {
  if(exp == 1) {
    return x;
  }
  if(exp % 2) {
    return (mod_pow(x, exp - 1) * x) % MOD;
  }
  ll y = mod_pow(x, exp / 2);
  return (y * y) % MOD;
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VL as(n);
  FOR(a,as) {
    cin >> a;
  }
  ll fact_n = 1;
  REP(i,1,n+1) {
    fact_n = (fact_n * i) % MOD;
  }
  VL inv(LIM);
  REP(i,1,LIM) {
    inv[i] = mod_pow(i, MOD-2);
  }
  VL inv_sum(LIM);
  inv_sum[0] = 0;
  REP(i,1,LIM) {
    inv_sum[i] = (inv_sum[i-1] + inv[i]) % MOD;
  }
  ll ans = 0;
  REP(i,0,n) {
    ll expect = (inv_sum[i+1] + inv_sum[n-i] - 1) % MOD;
    ll count = (expect * fact_n) % MOD;
    ll cost = (as[i] * count) % MOD;
    ans = (ans + cost) % MOD;
  }
  cout << ans << endl;

  return 0;
}

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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

ll gcd(ll x, ll y) {
  if(x == 0) {
    return y;
  }
  return gcd(y % x, x);
}

ll lcm(ll x, ll y) {
  ll g = gcd(x, y);
  return x / g * y;
}

int main() {
  ios::sync_with_stdio(false);
  ll N, Z;
  cin >> N >> Z;
  VL as(N);
  FOR(a, as) {
    cin >> a;
  }
  VL gcds(N);
  transform(as.begin(), as.end(), gcds.begin(), [=](ll a){return gcd(Z, a);});
  ll ans = 1;
  REP(i,0,N) {
    ans = lcm(ans, gcds[i]);
  }
  cout << ans << endl;

  return 0;
}

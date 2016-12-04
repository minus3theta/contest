#include <cstdio>
#include <climits>
#include <cassert>
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

ll solve(VL &as, ll x) {
  ll ans = 0;
  if(as[0] > x) {
    ans += as[0] - x;
    as[0] = x;
  }
  REP(i,0,as.size()-1) {
    ll s = as[i] + as[i+1];
    if(s > x) {
      ans += s - x;
      as[i+1] -= s - x;
      assert(as[i+1] >= 0);
    }
  }
  return ans;
}

int main() {
  ios::sync_with_stdio(false);
  int N;
  ll x;
  cin >> N >> x;
  VL as(N);
  FOR(a,as) {
    cin >> a;
  }
  // ll a1 = solve(as, x);
  // reverse(as.begin(), as.end());
  // ll a2 = solve(as, x);
  cout << solve(as,x) << endl;
  return 0;
}

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

ll solve(ll n, ll m, ll z, ll l, ll r, ll b) {
  ll seat = n * m;
  ll rest = n * (m + 1);
  ll occupied = z + l + r;
  if (occupied >= seat) {
    return seat;
  }
  rest -= l + r;
  b = min(b, seat - occupied);
  b = min(b, rest / 2);
  b = min(b, (m + 1) / 2 * n);
  return occupied + b;
}

int main() {
  ios::sync_with_stdio(false);
  int t;
  cin >> t;
  REP(i,0,t) {
    ll n, m, z, l, r, b;
    cin >> n >> m >> z >> l >> r >> b;
    cout << solve(n, m, z, l, r, b) << endl;
  }

  return 0;
}

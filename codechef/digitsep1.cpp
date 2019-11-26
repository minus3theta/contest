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
#include <unordered_set>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

ll gcd(ll x, ll y) {
  if (y == 0) {
    return x;
  }
  return gcd(y, x % y);
}

ll solve(int n, const string &s, int m, int x, int y) {
  vector<vector<unordered_set<ll>>> dp(n+1, vector<unordered_set<ll>>(y+2));
  dp[0][0].insert(0);
  REP(i,0,n) {
    REP(sep,0,y+1) {
      REP(prev,0,m) {
        if (prev > i) {
          break;
        }
        ll num = 0;
        REP(d,i-prev,i+1) {
          num *= 10;
          num += s[d] - '0';
        }
        FOR(val,dp[i-prev][sep]) {
          dp[i+1][sep+1].insert(gcd(val, num));
        }
      }
    }
  }
  ll ans = -1;
  REP(sep,x,y+1) {
    FOR(g,dp[n][sep+1]) {
      ans = max(ans,g);
    }
  }
  return ans;
}

int main() {
  ios::sync_with_stdio(false);
  int t;
  cin >> t;
  REP(i,0,t) {
    int n;
    string s;
    int m, x, y;
    cin >> n >> s >> m >> x >> y;
    cout << solve(n, s, m, x, y) << endl;
  }

  return 0;
}

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

constexpr ll MOD = 998244353;

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VI as(n);
  FOR(a,as) {
    cin >> a;
  }
  ll sum = accumulate(as.begin(), as.end(), 0ll);
  vector<VL> dp(n + 1, VL(sum + 1, 0ll));
  dp[0][0] = 1;
  REP(i,0,n) {
    REP(j,0,sum + 1) {
      dp[i+1][j] = dp[i][j] * 2 % MOD;
      if (j - as[i] >= 0) {
        dp[i+1][j] = (dp[i+1][j] + dp[i][j - as[i]]) % MOD;
      }
    }
  }
  ll ans = 1;
  REP(i,0,n) {
    ans = ans * 3 % MOD;
  }
  REP(j,(sum+1)/2,sum+1) {
    ans = (ans - (dp[n][j] * 3) % MOD + MOD) % MOD;
  }
  if (sum % 2 == 0) {
    vector<VL> dp_eq(n + 1, VL(sum / 2 + 1, 0ll));
    dp_eq[0][0] = 1;
    REP(i,0,n) {
      REP(j,0,sum / 2 + 1) {
        dp[i+1][j] = dp[i][j];
        if (j - as[i] >= 0) {
          dp[i+1][j] = (dp[i+1][j] + dp[i][j - as[i]]) % MOD;
        }
      }
    }
    ans = (ans + dp[n][sum / 2] * 3) % MOD;
  }
  cout << ans << endl;

  return 0;
}

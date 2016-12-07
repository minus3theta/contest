#include <cstdio>
#include <climits>
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

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

int main() {
  int N, A;
  cin >> N >> A;
  if(N > 1e6) return 0;
  VL dp(N+1,0);                   // dp[t] cookies at t
  ll ans = 0;
  REP(i,1,N+1) {
    dp[i] = i;
    for(ll j=1; i-j-A >= 0; j++) {
      // if(dp[i] < dp[i-j-A] * j) {
      //   cout << i << ", " << j << endl;
      // }
      dp[i] = max(dp[i], dp[i-j-A] * j);
    }
    // cout << i+1 << ", " << dp[i] << endl;
    if(dp[i] >= N) {
      ans = i;
      break;
    }
  }
  cout << ans << endl;;
  return 0;
}

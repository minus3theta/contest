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

struct person {
  ll h, p;
};

const ll INF = 1e10;

int main() {
  ios::sync_with_stdio(false);
  int N;
  cin >> N;
  vector<person> ps(N);
  FOR(p,ps) {
    cin >> p.h >> p.p;
  }
  sort(ps.begin(), ps.end(), [](const person &x, const person &y){
      return x.h + x.p < y.h + y.p;
    });
  vector<VL> dp(N+1, VL(N+1, INF));
  REP(i,0,N+1) {
    dp[i][0] = 0;
  }
  REP(i,1,N+1) {
    REP(j,1,N+1) {
      if(ps[i-1].h >= dp[i-1][j-1]) {
        dp[i][j] = min(dp[i-1][j], dp[i-1][j-1] + ps[i-1].p);
      } else {
        dp[i][j] = dp[i-1][j];
      }
    }
  }
  int ans = 0;
  REP(i,0,N+1) {
    if(dp[N][i] < INF) {
      ans = i;
    }
  }
  cout << ans << endl;
  
  return 0;
}

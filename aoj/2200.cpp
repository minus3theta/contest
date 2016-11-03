#include <cstdio>
#include <climits>
#include <iostream>
#include <string>
#include <algorithm>
#include <numeric>
#include <functional>
#include <list>
#include <map>
#include <queue>
#include <stack>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long ll;
typedef pair<ll,ll> PLL;

const ll INF = LONG_MAX/10LL;

int main() {
  int N, M;
  while(true) {
    cin >> N >> M;
    if(!N && !M) break;
    // cost of (land route, sea route) (1-indexed)
    vector<vector<PLL>> cost(N+1, vector<PLL>(N+1,{INF,INF}));
    REP(i,0,M) {
      int x, y, t;
      string s;
      cin >> x >> y >> t >> s;
      if(s == "L") {
        cost[x][y].first = t;
        cost[y][x].first = t;
      } else {
        cost[x][y].second = t;
        cost[y][x].second = t;
      }
    }
    REP(i,1,N+1) {
      cost[i][i] = {0,0};
    }
    int Z;
    cin >> Z;
    vector<int> r(Z);
    for(auto &x: r) {
      cin >> x;
    }
    // Warshall-Floyd
    REP(k,1,N+1) {
      REP(i,1,N+1) {
        REP(j,1,N+1) {
          cost[i][j].first = min(cost[i][j].first, cost[i][k].first + cost[k][j].first);
          cost[i][j].second = min(cost[i][j].second, cost[i][k].second + cost[k][j].second);
        }
      }
    }
    // dp[i][j]: cost to reach r[i], while the ship locates at j
    vector<vector<ll>> dp(Z, vector<ll>(N+1,INF));
    dp[0][r[0]] = 0;
    REP(i,1,Z) {
      REP(j,1,N+1) {
        REP(k,1,N+1) {
          dp[i][j] = min(dp[i][j], dp[i-1][k] + cost[r[i-1]][k].first +
                         cost[k][j].second + cost[j][r[i]].first);
        }
      }
    }
    ll mc = INF;
    REP(i,1,N+1) {
      mc = min(mc, dp[Z-1][i]);
    }
    cout << mc << endl;
  }
  return 0;
}

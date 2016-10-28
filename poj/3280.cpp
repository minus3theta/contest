#include <cstdio>
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

const int INF = 10000 * 3000;

int main() {
  int n, m;
  string s;
  cin >> n >> m >> s;
  vector<int> cost(26);
  REP(i,0,n) {
    char c;
    cin >> c;
    int x, y;
    cin >> x >> y;
    cost[c-'a'] = min(x, y);
  }
  string t = s;
  reverse(t.begin(), t.end());
  int l = s.size();
  vector<vector<int> > dp(l+2);
  REP(i,0,l+2) {
    dp[i].resize(l+2);
    dp[i][0] = INF;
  }
  REP(j,0,l+2) {
    dp[0][j] = INF;
  }
  dp[1][1] = 0;
  REP(i,2,l+2) {
    dp[i][1] = dp[i-1][1] + cost[s[i-2]-'a'];
    dp[1][i] = dp[1][i-1] + cost[t[i-2]-'a'];
  }

  REP(i,2,l+2) {
    REP(j,2,l+2) {
      int x = INF;
      if(s[i-2] == t[j-2]) {
        x = dp[i-1][j-1];
      }
      dp[i][j] = min(x, min(dp[i-1][j] + cost[s[i-2]-'a'],
                     dp[i][j-1] + cost[t[j-2]-'a']));
    }
  }
  cout << dp[l+1][l+1] / 2 << endl;;
  return 0;
}

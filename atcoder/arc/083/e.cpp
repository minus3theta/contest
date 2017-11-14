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

const int INF = 1 << 30;

struct vtx {
  int x;
  VI ch;
  int op;
};

bool dfs(vector<vtx> &vs, int p) {
  vtx &v = vs[p];
  if(v.ch.size() == 0) {
    v.op = 0;
    return true;
  }
  FOR(c,v.ch) {
    if(!dfs(vs, c)) {
      return false;
    }
  }
  vector<VI> dp(v.ch.size() + 1, VI(v.x + 1, INF));
  dp[0][0] = 0;
  REP(i,0,v.ch.size()) {
    vtx &c = vs[v.ch[i]];
    REP(j,0,v.x+1) {
      if(c.x <= j) {
        dp[i+1][j] = min(dp[i+1][j], dp[i][j-c.x] + c.op);
      }
      if(c.op <= j) {
        dp[i+1][j] = min(dp[i+1][j], dp[i][j-c.op] + c.x);
      }
    }
  }
  v.op = INF;
  FOR(a,dp[v.ch.size()]) {
    v.op = min(v.op, a);
  }
  return v.op != INF;
}

int main() {
  ios::sync_with_stdio(false);
  int N;
  cin >> N;
  vector<vtx> vs(N);
  REP(i,1,N) {
    int j;
    cin >> j;
    vs[j-1].ch.push_back(i);
  }
  FOR(v,vs) {
    cin >> v.x;
  }
  cout << (dfs(vs, 0) ? "POSSIBLE" : "IMPOSSIBLE") << endl;;

  return 0;
}

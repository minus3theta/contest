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

const int INF = 1e9;

struct query {
  int v, d, c;
};

int main() {
  ios::sync_with_stdio(false);
  int N, M;
  cin >> N >> M;
  vector<VI> es(N+1);
  REP(i,0,M) {
    int a, b;
    cin >> a >> b;
    es[a].push_back(b);
    es[b].push_back(a);
  }
  int Q;
  cin >> Q;
  vector<query> qs(Q);
  FOR(q,qs) {
    cin >> q.v >> q.d >> q.c;
  }
  reverse(qs.begin(), qs.end());
  vector<VI> dp(N+1, VI(11, INF));
  REP(i,0,Q) {
    const auto q = qs[i];
    dp[q.v][q.d] = min(dp[q.v][q.d], i);
  }
  for(int i=9; i>=0; i--) {
    REP(x,1,N+1) {
      dp[x][i] = min(dp[x][i], dp[x][i+1]);
      FOR(y,es[x]) {
        dp[x][i] = min(dp[x][i], dp[y][i+1]);
      }
    }
  }
  REP(x,1,N+1) {
    if(dp[x][0] == INF) {
      cout << 0 << endl;
    } else {
      cout << qs[dp[x][0]].c << endl;
    }
  }
  return 0;
}

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

void dfs(int x, VI &dist, const vector<VI> &es) {
  FOR(y,es[x]) {
    if(dist[y] < 0) {
      dist[y] = dist[x] + 1;
      dfs(y, dist, es);
    }
  }
}

int main() {
  ios::sync_with_stdio(false);
  int N;
  cin >> N;
  vector<VI> es(N);
  REP(i,0,N-1) {
    int a, b;
    cin >> a >> b;
    es[--a].push_back(--b);
    es[b].push_back(a);
  }
  vector<VI> dist(2, VI(N, -1));
  dist[0][0] = 0;
  dfs(0, dist[0], es);
  dist[1][N-1] = 0;
  dfs(N-1, dist[1], es);
  int fen = 0;
  REP(i,0,N) {
    if(dist[0][i] <= dist[1][i]) {
      fen++;
    }
  }
  cout << (fen > (N - fen) ? "Fennec" : "Snuke") << endl;
 
  return 0;
}

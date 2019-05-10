#include <cstdio>
#include <climits>
#include <cassert>
#include <cmath>
#include <iostream>
#include <iomanip>
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
typedef pair<double, int> PDI;

struct bar {
  ll x, y, r;
};

int main() {
  ios::sync_with_stdio(false);
  bar s, t;
  cin >> s.x >> s.y >> t.x >> t.y;
  int N;
  cin >> N;
  vector<bar> ps(N);
  FOR(p,ps) {
    cin >> p.x >> p.y >> p.r;
  }
  ps.push_back({s.x, s.y, 0});
  ps.push_back({t.x, t.y, 0});
  vector<vector<double>> cost(N+2, vector<double>(N+2));
  REP(i,0,N+2) {
    REP(j,0,N+2) {
      if(i == j) {
        cost[i][j] = 0.0;
      } else {
        double dst = sqrt((ps[i].x - ps[j].x) * (ps[i].x - ps[j].x) +
                          (ps[i].y - ps[j].y) * (ps[i].y - ps[j].y));
        cost[i][j] = max(0.0, dst - ps[i].r - ps[j].r);
      }
    }
  }
  priority_queue<PDI, vector<PDI>, greater<PDI>> que;
  que.emplace(0.0, N);
  vector<bool> visited(N+2, false);
  vector<double> dist(N+2, 1e30);
  dist[N] = 0.0;
  while (!que.empty()) {
    int current = que.top().second;
    que.pop();
    if (visited[current]) {
      continue;
    }
    visited[current] = true;
    REP(i,0,N+2) {
      double d = dist[current] + cost[current][i];
      if (d < dist[i]) {
        dist[i] = d;
        que.emplace(d, i);
      }
    }
  }

  cout << setprecision(12) << dist[N+1] << endl;
  return 0;
}

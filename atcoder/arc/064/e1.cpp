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
  vector<bool> visited(N+2, false);
  int current = N;
  while (!visited[N+1]) {
    visited[current] = true;
    int next = -1;
    double next_cost = 1e30;
    REP(i,0,N+2) {
      if (!visited[i] && cost[N][i] < next_cost) {
        next_cost = cost[N][i];
        next = i;
      }
    }
    if (next_cost == 1e30) {
      break;
    }
    current = next;
    REP(i,0,N+2) {
      cost[N][i] = min(cost[N][i], cost[N][current] + cost[current][i]);
    }
  }

  cout << setprecision(12) << cost[N][N+1] << endl;
  return 0;
}

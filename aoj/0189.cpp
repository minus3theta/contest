#include <cstdio>
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

const int INF = 1<<20;

struct edge {
  int s, t, c;
};

int main() {
  int n;
  while(true) {
    cin >> n;
    if(!n) break;
    vector<edge> es(n);
    REP(i,0,n) {
      cin >> es[i].s >> es[i].t >> es[i].c;
    }
    int m = 0;
    for(auto &e: es) {
      m = max(m, max(e.s,e.t));
    }
    m++;
    vector<vector<int>> cost(m, vector<int>(m,INF));
    for(auto &e: es) {
      cost[e.s][e.t] = e.c;
      cost[e.t][e.s] = e.c;
    }
    REP(i,0,m) {
      cost[i][i] = 0;
    }
    REP(k,0,m) {
      REP(i,0,m) {
        REP(j,0,m) {
          cost[i][j] = min(cost[i][j], cost[i][k] + cost[k][j]);
        }
      }
    }
    vector<int> sum(m,0);
    int mini = -1;
    int mind = INF;
    REP(i,0,m) {
      REP(j,0,m) {
        sum[i] += cost[i][j];
      }
      if(sum[i] < mind) {
        mind = sum[i];
        mini = i;
      }
    }
    cout << mini << " " << mind << endl;
  }
  return 0;
}

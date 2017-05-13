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

struct edge {
  int src, dst;
  ll cost;
};

int main() {
  ios::sync_with_stdio(false);
  int N, M;
  cin >> N >> M;
  vector<edge> es(M);
  FOR(e,es) {
    cin >> e.src >> e.dst >> e.cost;
    e.src--;
    e.dst--;
  }
  VL score(N, -1e13);
  score[0] = 0;
  bool inf = false;
  REP(i,0,N) {
    FOR(e,es) {
      if(score[e.dst] < score[e.src] + e.cost) {
        score[e.dst] = score[e.src] + e.cost;
        if(i == N - 1 && e.dst == N - 1) {
          inf = true;
          break;
        }
      }
    }
  }
  if(inf) {
    cout << "inf" << endl;
  } else {
    cout << score[N - 1] << endl;
  }
  return 0;
}

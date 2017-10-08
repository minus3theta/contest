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

struct vtx {
  VI edge;
  int color;
};

bool color(vector<vtx> &vs, int x, int c) {
  if(vs[x].color != 0) {
    return vs[x].color != c;
  }
  vs[x].color = c;
  FOR(y, vs[x].edge) {
    if(color(vs, y, 3-c)) {
      return true;
    }
  }
  return false;
}

int main() {
  ios::sync_with_stdio(false);
  ll N, M;
  cin >> N >> M;
  vector<vtx> es(N, {VI(), 0});
  REP(i,0,M) {
    int a, b;
    cin >> a >> b;
    es[a-1].edge.push_back(b-1);
    es[b-1].edge.push_back(a-1);
  }
  ll total = 0;
  if(color(es, 0, 1)) {
    total = N * (N-1) / 2;
  } else {
    int c1 = 0;
    FOR(v,es) {
      if(v.color == 1) {
        c1++;
      }
    }
    total = c1 * (N - c1);
  }
  cout << total - M << endl;

  return 0;
}

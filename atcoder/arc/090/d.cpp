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
#include <iterator>
#include <regex>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

template <class T, template <class, class> class C, class charT = char>
void vdump(const C<T, allocator<T>> &v, const charT* delimiter = ", ",
           ostream &stream = cout) {
  copy(v.begin(), v.end(), ostream_iterator<T>(stream, delimiter));
  stream << endl;
}

const int INF = 2e9;

struct edge {
  int dst;
  int d;
};

struct vtx {
  int x = INF;
  vector<edge> es;
};

bool dfs(vector<vtx> &vs, int a) {
  vtx &v = vs[a];
  FOR(e,v.es) {
    vtx &u = vs[e.dst];
    if(u.x == INF) {
      u.x = v.x + e.d;
      if(!dfs(vs, e.dst)) {
        return false;
      }
    } else {
      if(u.x != v.x + e.d) {
        return false;
      }
    }
  }
  return true;
}

bool solve(vector<vtx> &vs) {
  int n = vs.size();
  REP(i,0,n) {
    if(vs[i].x != INF) {
      continue;
    }
    vs[i].x = 0;
    if(!dfs(vs, i)) {
      return false;
    }
  }
  return true;
}

int main() {
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  vector<vtx> vs(n);
  REP(i,0,m) {
    int l, r, d;
    cin >> l >> r >> d;
    l--;
    r--;
    vs[l].es.push_back({r, d});
    vs[r].es.push_back({l, -d});
  }
  cout << (solve(vs) ? "Yes" : "No") << endl;

  return 0;
}

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

void dfs(const vector<vector<pair<int,ll>>> &vs, VI &color, int v, int col) {
  if (color[v] >= 0) return;
  color[v] = col;
  FOR(e,vs[v]) {
    int u = e.first;
    ll w = e.second;
    int ncol = w % 2 == 0 ? col : 1 - col;
    dfs(vs, color, u, ncol);
  }
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  vector<vector<pair<int,ll>>> vs(n);
  REP(i,0,n-1) {
    int u, v;
    ll w;
    cin >> u >> v >> w;
    vs[u-1].emplace_back(v-1, w);
    vs[v-1].emplace_back(u-1, w);
  }
  VI color(n, -1);
  dfs(vs, color, 0, 0);
  FOR(c,color) {
    cout << c << endl;
  }

  return 0;
}

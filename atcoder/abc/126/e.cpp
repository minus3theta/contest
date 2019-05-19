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

void dfs(const vector<VI> &vs, vector<bool> &visited, int v) {
  visited[v] = true;
  FOR(u,vs[v]) {
    if (!visited[u]) {
      dfs(vs, visited, u);
    }
  }
}

int main() {
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  vector<VI> vs(n);
  REP(i,0,m) {
    int x, y, z;
    cin >> x >> y >> z;
    vs[x-1].push_back(y-1);
    vs[y-1].push_back(x-1);
  }
  vector<bool> visited(n, false);
  int ans = 0;
  REP(i,0,n) {
    if (!visited[i]) {
      ans++;
      dfs(vs, visited, i);
    }
  }
  cout << ans << endl;

  return 0;
}

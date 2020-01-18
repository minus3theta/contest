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

using T=pair<ll,PI>;
void dijk(int h, int w, const vector<VL> &a, vector<VL> &dist, int si, int sj) {
  int di[] = {0, 1, 0, -1};
  int dj[] = {1, 0, -1, 0};
  priority_queue<T, vector<T>, greater<T>> q;
  q.push({0, {si, sj}});
  while (! q.empty()) {
    ll d = q.top().first;
    PI p = q.top().second;
    q.pop();
    int i = p.first;
    int j = p.second;
    REP(k,0,4) {
      int ni = i + di[k];
      int nj = j + dj[k];
      if (ni < 0 || ni >= h || nj < 0 || nj > w) {
        continue;
      }
      if (d + a[ni][nj] < dist[ni][nj]) {
        dist[ni][nj] = d + a[ni][nj];
        q.push({dist[ni][nj], {ni, nj}});
      }
    }
  }
}

int main() {
  ios::sync_with_stdio(false);
  int h, w;
  cin >> h >> w;
  vector<VL> a(h, VL(w));
  FOR(v,a) {
    FOR(x,v) {
      cin >> x;
    }
  }
  vector<vector<VL>> dist(3, vector<VL>(h, VL(w, 1e18)));
  dist[0][h-1][0] = 0;
  dist[1][h-1][w-1] = 0;
  dist[2][0][w-1] = 0;
  dijk(h, w, a, dist[0], h-1, 0);
  dijk(h, w, a, dist[1], h-1, w-1);
  dijk(h, w, a, dist[2], 0, w-1);
  ll ans = 1e18;
  REP(i,0,h) {
    REP(j,0,w) {
      ans = min(ans, dist[0][i][j] + dist[1][i][j] + dist[2][i][j] - 2 * a[i][j]);
    }
  }
  cout << ans << endl;

  return 0;
}

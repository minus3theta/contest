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

int main() {
  ios::sync_with_stdio(false);
  int n, m, r;
  cin >> n >> m >> r;
  VI rs(r);
  FOR(x,rs) {
    cin >> x;
    x--;
  }
  vector<VL> dist(n, VL(n, 1e18));
  REP(i,0,m) {
    int a, b;
    ll c;
    cin >> a >> b >> c;
    dist[a-1][b-1] = c;
    dist[b-1][a-1] = c;
  }
  REP(k,0,n) {
    REP(i,0,n) {
      REP(j,0,n) {
        dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j]);
      }
    }
  }
  sort(rs.begin(), rs.end());
  ll ans = 1e18;
  do {
    ll d = 0;
    REP(i,1,r) {
      d += dist[rs[i-1]][rs[i]];
    }
    ans = min(ans, d);
  } while (next_permutation(rs.begin(), rs.end()));
  cout << ans << endl;

  return 0;
}

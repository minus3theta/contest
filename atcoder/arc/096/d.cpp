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

ll solve(const vector<PL> &xs, const vector<PL> &rs) {
  int n = xs.size();
  VL gain(n+1, 0);
  VL rgain(n+1, 0);
  REP(i,0,n) {
    gain[i+1] = gain[i] + xs[i].second;
    rgain[i+1] = rgain[i] + rs[i].second;
  }
  REP(i,0,n) {
    gain[i+1] -= xs[i].first;
    rgain[i+1] -= 2 * rs[i].first;
  }
  ll rmax = 0;
  ll ans = 0;
  REP(i,0,n) {
    rmax = max(rmax, rgain[i]);
    ans = max(ans, gain[n-i] + rmax);
  }
  // vdump(gain);
  // vdump(rgain);
  return ans;
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  ll c;
  cin >> n >> c;
  vector<PL> xs(n);
  FOR(x,xs) {
    cin >> x.first >> x.second;
  }
  vector<PL> rs(n);
  transform(xs.begin(), xs.end(), rs.rbegin(), [=](PL x){return make_pair(c-x.first, x.second);});
  ll l1 = solve(xs, rs);
  ll l2 = solve(rs, xs);
  cout << max(l1, l2) << endl;

  return 0;
}

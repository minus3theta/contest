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

ll solve(const VL &ss, const VL &ts, ll q) {
  int sei = lower_bound(ss.begin(), ss.end(), q) - ss.begin();
  int tei = lower_bound(ts.begin(), ts.end(), q) - ts.begin();
  ll se = ss[sei];
  ll sw = ss[sei-1];
  ll te = ts[tei];
  ll tw = ts[tei-1];
  ll ans = 1ll << 60;
  ans = min(ans, q - min(sw, tw));
  ans = min(ans, max(se, te) - q);
  ans = min(ans, se - tw + min(se - q, q - tw));
  ans = min(ans, te - sw + min(te - q, q - sw));
  return ans;
}

int main() {
  ios::sync_with_stdio(false);
  int a, b, qc;
  cin >> a >> b >> qc;
  VL ss = {-(1ll << 60)};
  REP(i,0,a) {
    ll s;
    cin >> s;
    ss.push_back(s);
  }
  ss.push_back(1ll << 60);
  VL ts = {-(1ll << 60)};
  REP(i,0,b) {
    ll t;
    cin >> t;
    ts.push_back(t);
  }
  ts.push_back(1ll << 60);
  VL qs(qc);
  FOR(q,qs) {
    cin >> q;
  }
  FOR(q,qs) {
    cout << solve(ss, ts, q) << endl;
  }

  return 0;
}

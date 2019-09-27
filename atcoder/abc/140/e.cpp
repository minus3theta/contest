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
  int n;
  cin >> n;
  VI ps(n);
  FOR(p, ps) {
    cin >> p;
  }
  VI rev(n+1);
  REP(i,0,n) {
    rev[ps[i]] = i + 1;
  }
  multiset<int> pos = {0, 0, n + 1, n + 1};
  ll ans = 0;
  for (int i = n; i >= 1; i--) {
    int p = rev[i];
    auto it = pos.lower_bound(p);
    ll r1 = *it;
    ll r2 = *++it;
    --it;
    ll l1 = *--it;
    ll l2 = *--it;
    ans += i * ((p - l1) * (r2 - r1) + (r1 - p) * (l1 - l2));
    pos.insert(p);
  }

  cout << ans << endl;

  return 0;
}

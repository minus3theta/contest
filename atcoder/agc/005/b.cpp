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
  VI pos(n);
  REP(i,0,n) {
    int a;
    cin >> a;
    pos[a-1] = i;
  }
  set<int> idx = {-1, n};
  ll ans = 0;
  REP(i,0,n) {
    int p = pos[i];
    auto it = idx.lower_bound(p);
    ll r = *it;
    ll l = *--it;
    ans += (i + 1) * (r - p) * (p - l);
    idx.insert(p);
  }
  cout << ans << endl;

  return 0;
}

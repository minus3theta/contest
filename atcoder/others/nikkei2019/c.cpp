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
  vector<tuple<ll, ll, ll>> ds(n);
  REP(i,0,n) {
    ll a, b;
    cin >> a >> b;
    ds[i] = make_tuple(a + b, a, b);
  }
  sort(ds.rbegin(), ds.rend());
  ll ans = 0;
  REP(i,0,n) {
    if(i % 2 == 0) {
      ans += get<1>(ds[i]);
    } else {
      ans -= get<2>(ds[i]);
    }
  }
  cout << ans << endl;

  return 0;
}

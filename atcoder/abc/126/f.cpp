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
  ll m, k;
  cin >> m >> k;
  if (k >= (1 << m)) {
    cout << -1 << endl;
    return 0;
  }
  if (m == 0) {
    cout << "0 0" << endl;
    return 0;
  }
  if (m == 1) {
    if (k == 0) {
      cout << "0 0 1 1" << endl;
    } else {
      cout << -1 << endl;
    }
    return 0;
  }
  cout << k << " ";
  for (ll i=0; i < 1 << m; i++) {
    if (i != k) {
      cout << i << " ";
    }
  }
  cout << k << " ";
  for (ll i=(1 << m) - 1; i >= 0; i--) {
    if (i != k) {
      cout << i << " ";
    }
  }
  cout << endl;

  return 0;
}

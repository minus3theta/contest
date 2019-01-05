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
  int n, a, b;
  cin >> n >> a >> b;
  if (n < a + b - 1 || (ll)a * (ll)b < (ll)n) {
    cout << -1 << endl;
    return 0;
  }
  VI vs(a, 1);
  int m = n - a;
  FOR(v, vs) {
    if (m <= 0) {
      break;
    }
    int d = min(m, b - v);
    m -= d;
    v += d;
  }
  int i = 0;
  FOR(v, vs) {
    REP(j,0,v) {
      cout << i + v - j << " ";
    }
    i += v;
  }
  cout << endl;

  return 0;
}

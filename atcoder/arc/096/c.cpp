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
  ll a, b, c, x, y;
  cin >> a >> b >> c >> x >> y;
  if(a + b < 2 * c) {
    cout << x * a + y * b << endl;
  } else {
    if(x > y) {
      ll s = y * 2 * c;
      s += min(a, 2 * c) * (x - y);
      cout << s << endl;
    } else {
      ll s = x * 2 * c;
      s += min(b, 2 * c) * (y - x);
      cout << s << endl;
    }
  }

  return 0;
}

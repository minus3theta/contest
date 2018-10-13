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

ll gcd(ll x, ll y) {
  if(y == 0) {
    return x;
  }
  return gcd(y, x % y);
}

ll lcm(ll x, ll y) {
  return x / gcd(x, y) * y;
}

int main() {
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  string s, t;
  cin >> s >> t;
  ll g = gcd(n, m);
  bool ok = true;
  REP(i,0,g) {
    if(s[n / g * i] != t[m / g * i]) {
      ok = false;
      break;
    }
  }
  if(ok) {
    cout << lcm(n, m) << endl;
  } else {
    cout << -1 << endl;
  }

  return 0;
}

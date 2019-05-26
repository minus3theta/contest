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
  int n, m;
  cin >> n >> m;
  vector<VI> bulb(m);
  FOR(b,bulb) {
    int k;
    cin >> k;
    REP(i,0,k) {
      int s;
      cin >> s;
      b.push_back(s);
    }
  }
  VI ps(m);
  FOR(p,ps) {
    cin >> p;
  }
  ll ans = 0;
  REP(i,0,1<<n) {
    bool ok = true;
    REP(j,0,m) {
      bool on = ps[j] == 0;
      FOR(s,bulb[j]) {
        if ((i >> (s - 1) & 1) != 0) {
          on = !on;
        }
      }
      if (!on) {
        ok = false;
        break;
      }
    }
    if (ok) {
      ans++;
    }
  }
  cout << ans << endl;

  return 0;
}

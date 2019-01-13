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

constexpr ll MOD = 1e9 + 7;

int main() {
  ios::sync_with_stdio(false);
  ll n, m;
  cin >> n >> m;
  VI as(n), bs(m);
  FOR(a,as) {
    cin >> a;
  }
  FOR(b,bs) {
    cin >> b;
  }
  VI popa(n * m + 1, 0), popb(n * m + 1, 0);
  FOR(a,as) {
    popa[a]++;
  }
  FOR(b,bs) {
    popb[b]++;
  }
  ll ans = 1;
  ll useda = 0;
  ll usedb = 0;
  for(int i = n * m; i > 0; i--) {
    if (popa[i] > 1 || popb[i] > 1) {
      ans = 0;
      break;
    }
    if (popa[i] == 1) {
      useda++;
      if (popb[i] == 1) {
        usedb++;
      } else {
        ans = (ans * usedb) % MOD;
      }
    } else {
      if (popb[i] == 1) {
        usedb++;
        ans = (ans * useda) % MOD;
      } else {
        ll space = useda * usedb;
        ll cap = space - (n * m - i);
        if (cap <= 0) {
          ans = 0;
          break;
        }
        ans = (ans * cap) % MOD;
      }
    }
  }
  cout << ans << endl;

  return 0;
}

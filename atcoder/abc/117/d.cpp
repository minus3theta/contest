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
  ll k;
  cin >> n >> k;
  VL as(n);
  FOR(a,as) {
    cin >> a;
  }
  VI pop(40, 0);
  FOR(a,as) {
    REP(i,0,40) {
      if ((a >> i) & 1) {
        pop[i]++;
      }
    }
  }
  ll maxf = 0;
  FOR(a,as) {
    maxf += a ^ k;
  }
  REP(i,0,40) {
    if (!((k >> i) & 1)) {
      continue;
    }
    ll f = 0;
    REP(j,0,40) {
      if (j < i) {
        f += max(pop[j], n - pop[j]) * (1ll << j);
      } else if (j == i) {
        f += pop[j] * (1ll << j);
      } else {
        if ((k >> j) & 1) {
          f += (n - pop[j]) * (1ll << j);
        } else {
          f += pop[j] * (1ll << j);
        }
      }
    }
    maxf = max(maxf, f);
  }
  cout << maxf << endl;

  return 0;
}

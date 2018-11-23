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

bool inc(int n, int i, int j) {
  return n <= 2 * (i + j) && 2 * (i + j) <= 3 * n && -n <= 2 * (i - j) && 2 * (i - j) <= n;
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  int ans = 0;
  REP(i,0,n) {
    REP(j,0,n) {
      if(inc(n, i, j) && inc(n, i, j + 1) && inc(n, i + 1, j) && inc(n, i + 1, j + 1)) {
        ans++;
      }
    }
  }
  cout << ans << endl;

  return 0;
}

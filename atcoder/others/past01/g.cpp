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

bool same(int g, int i, int j) {
  return ((g & (1 << i)) != 0) && ((g & (1 << j)) != 0);
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  vector<VL> mat(n, VL(n));
  REP(i,0,n-1) {
    REP(j,i+1,n) {
      ll a;
      cin >> a;
      mat[i][j] = a;
    }
  }
  // REP(i,0,n) {
  //   REP(j,0,n) {
  //     cout << mat[i][j] << " ";
  //   }
  //   cout << endl;
  // }
  ll ans = -1e18;
  REP(a,0,1<<n) {
    REP(b,0,1<<n) {
      if ((a & b) != 0) {
        continue;
      }
      int c = ((1 << n) - 1) ^ (a | b);
      // assert((a & c) == 0);
      // assert((b & c) == 0);
      // assert((a | b | c) == ((1 << n) - 1));
      // printf("%x %x %x\n", a, b, c);
      ll score = 0;
      REP(i,0,n) {
        REP(j,i+1,n) {
          if (same(a, i, j) || same(b, i, j) || same(c, i, j)) {
            score += mat[i][j];
          }
        }
      }
      ans = max(ans, score);
    }
  }
  cout << ans << endl;

  return 0;
}

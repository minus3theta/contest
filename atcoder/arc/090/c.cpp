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
  vector<VI> as(2, VI(n));
  FOR(a,as) {
    FOR(x,a) {
      cin >> x;
    }
  }
  REP(i,1,n) {
    as[0][i] += as[0][i-1];
  }
  for(int i=n-2; i>=0; i--) {
    as[1][i] += as[1][i+1];
  }

  int ans = -1;
  REP(i,0,n) {
    ans = max(ans, as[0][i] + as[1][i]);
  }
  cout << ans << endl;
  
  return 0;
}

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
  string s;
  cin >> s;
  VI e(n+1);
  VI w(n+1);
  REP(i,0,n) {
    e[i+1] = e[i] + (s[i] == 'W');
  }
  for(int i=n-1; i>=0; i--) {
    w[i] = w[i+1] + (s[i] == 'E');
  }
  int ans = 1e8;
  REP(i,0,n) {
    ans = min(ans, e[i] + w[i+1]);
  }
  cout << ans << endl;

  return 0;
}

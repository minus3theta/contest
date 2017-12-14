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
  ll x;
  cin >> n >> x;
  VL ts(n);
  FOR(t,ts) {
    cin >> t;
  }
  vector<VL> dp(n, VL(n+1, 0));
  dp[0][1] = x;
  REP(i,0,n-1) {
    int j = i + 1;
    while(j < n && ts[j] <= ts[i] + x) {
      j++;
    }
    REP(k,1,n) {
      dp[j-1][k+1] = max(dp[j-1][k+1], dp[i][k] + ts[j-1] - ts[i]);
      if(j < n) {
        dp[j][k+1] = max(dp[j][k+1], dp[i][k] + x);
      }
    }
  }
  ll s = 0;
  REP(k,1,n+1) {
    REP(i,0,n) {
      s = max(s, dp[i][k]);
    }
    cout << s << endl;
  }

  return 0;
}

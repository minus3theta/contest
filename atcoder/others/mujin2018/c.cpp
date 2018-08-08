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
  vector<string> ss(n);
  FOR(s,ss) {
    cin >> s;
  }
  vector<vector<VI>> margin(4, vector<VI>(n, VI(m, 0)));
  REP(i,0,n) {
    REP(j,1,m) {
      if(ss[i][j-1] == '.') {
        margin[0][i][j] = margin[0][i][j-1] + 1;
      }
    }
  }
  for(int i=n-2; i>=0; i--) {
    REP(j,0,m) {
      if(ss[i+1][j] == '.') {
        margin[1][i][j] = margin[1][i+1][j] + 1;
      }
    }
  }
  REP(i,0,n) {
    for(int j=m-2; j>=0; j--) {
      if(ss[i][j+1] == '.') {
        margin[2][i][j] = margin[2][i][j+1] + 1;
      }
    }
  }
  REP(i,1,n) {
    REP(j,0,m) {
      if(ss[i-1][j] == '.') {
        margin[3][i][j] = margin[3][i-1][j] + 1;
      }
    }
  }
  ll ans = 0;
  REP(i,0,n) {
    REP(j,0,m) {
      REP(k,0,4) {
        if(ss[i][j] == '#') continue;
        ans += margin[k][i][j] * margin[(k+1)%4][i][j];
      }
    }
  }
  cout << ans << endl;

  return 0;
}

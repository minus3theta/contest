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

const ll MOD = 1e9+7;

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  string s, t;
  cin >> s >> t;
  ll ans;
  bool vertical;
  int i;
  if((vertical = s[0] == t[0])) {
    ans = 3;
    i = 1;
  } else {
    ans = 6;
    i = 2;
  }
  for(; i<n; i++) {
    if(s[i] == t[i]) {
      if(vertical) {
        ans = ans * 2 % MOD;
      }
      vertical = true;
    } else {
      if(vertical) {
        ans = ans * 2 % MOD;
      } else {
        ans = ans * 3 % MOD;
      }
      i++;
      vertical = false;
    }
  }
  cout << ans << endl;
  
  return 0;
}

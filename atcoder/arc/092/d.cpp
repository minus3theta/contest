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
  VI as(n);
  VI bs(n);
  FOR(a,as) {
    cin >> a;
  }
  FOR(b,bs) {
    cin >> b;
  }
  int ans = 0;
  REP(i,0,30) {
    int mod = 2 << i;
    VI b_mod(n);
    transform(bs.begin(), bs.end(), b_mod.begin(), [=](int x){return x % mod;});
    sort(b_mod.begin(), b_mod.end());
    int count = 0;
    FOR(a,as) {
      int ak = a % mod;
      int ub = mod - 1 - ak;
      int lb = ub - mod / 2 + 1;
      count += upper_bound(b_mod.begin(), b_mod.end(), ub) - b_mod.begin();
      if(lb < 0) {
        count += n;
        lb += mod;
      }
      count -= lower_bound(b_mod.begin(), b_mod.end(), lb) - b_mod.begin();
    }
    if(count % 2) {
      ans |= 1 << i;
    }
  }
  cout << ans << endl;
  
  return 0;
}

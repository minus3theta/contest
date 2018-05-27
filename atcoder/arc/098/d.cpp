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
  FOR(a,as) {
    cin >> a;
  }
  vector<VI> include(20);
  REP(i,0,n) {
    REP(b,0,20) {
      if(((as[i] >> b) & 1) != 0) {
        include[b].push_back(i);
      }
    }
  }
  ll ans = 0;
  int first = -1;
  REP(i,0,n) {
    REP(b,0,20) {
      if(((as[i] >> b) & 1) != 0) {
        auto lb = lower_bound(include[b].begin(), include[b].end(), i);
        if(lb != include[b].begin()) {
          first = max(first, *(lb-1));
        }
      }
    }
    ans += i - first;
  }
  cout << ans << endl;
  
  return 0;
}

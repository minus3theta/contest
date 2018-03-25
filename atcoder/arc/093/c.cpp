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
  VL as(n+2, 0);
  REP(i,0,n) {
    cin >> as[i+1];
  }
  ll total = 0;
  REP(i,0,n+1) {
    total += abs(as[i+1] - as[i]);
  }
  REP(i,1,n+1) {
    if((as[i-1] <= as[i] && as[i] <= as[i+1]) ||
       (as[i-1] >= as[i] && as[i] >= as[i+1])) {
      cout << total << endl;
    } else {
      ll cost = min(abs(as[i] - as[i-1]), abs(as[i] - as[i+1]));
      cout << total - 2 * cost << endl;
    }
  }
  
  return 0;
}

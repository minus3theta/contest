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
  VL as(n);
  VL bs(n);
  FOR(a,as) {
    cin >> a;
  }
  FOR(b,bs) {
    cin >> b;
  }
  ll lack = 0;
  int lc = 0;
  VL cap;
  REP(i,0,n) {
    if (as[i] < bs[i]) {
      lack += bs[i] - as[i];
      lc++;
    } else if(as[i] > bs[i]) {
      cap.push_back(as[i] - bs[i]);
    }
  }
  sort(cap.rbegin(), cap.rend());
  int ans = lc;
  FOR(c,cap) {
    if (lack <= 0) {
      break;
    }
    lack -= c;
    ans++;
  }
  if (lack > 0) {
    cout << -1 << endl;
  } else {
    cout << ans << endl;
  }

  return 0;
}

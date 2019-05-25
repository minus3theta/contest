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
  int qs;
  cin >> qs;
  multiset<ll, greater<ll>> left;
  left.insert(-1e18);
  multiset<ll> right;
  right.insert(1e18);
  ll y = 0;
  REP(i,0,qs) {
    int t;
    cin >> t;
    if (t == 2) {
      cout << *left.begin() << " " << y << endl;
    } else {
      ll a, b;
      cin >> a >> b;
      y += b;
      auto li = left.begin();
      auto ri = right.begin();
      if (*li <= a && a <= *ri) {
        left.insert(a);
        right.insert(a);
      } else if (a < *li) {
        y += *li - a;
        right.insert(*li);
        left.erase(li);
        left.insert(a);
        left.insert(a);
      } else {
        y += a - *ri;
        left.insert(*ri);
        right.erase(ri);
        right.insert(a);
        right.insert(a);
      }
    }
  }

  return 0;
}

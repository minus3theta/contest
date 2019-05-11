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
  vector<string> ss(n);
  FOR(s,ss) {
    cin >> s;
  }
  int begin_b = 0;
  int end_a = 0;
  int both = 0;
  FOR(s,ss) {
    if (s[0] == 'B') {
      if (s.back() == 'A') {
        both++;
      } else {
        begin_b++;
      }
    } else if (s.back() == 'A') {
      end_a++;
    }
  }
  int ans = 0;
  if (both > 0) {
    ans += both - 1;
    if (end_a > 0) {
      ans += 1;
      end_a--;
    }
    if (begin_b > 0) {
      ans += 1;
      begin_b--;
    }
  }
  ans += min(begin_b, end_a);
  FOR(s,ss) {
    REP(i,0,s.size()-1) {
      if (s[i] == 'A' && s[i+1] == 'B') {
        ans++;
      }
    }
  }
  cout << ans << endl;

  return 0;
}

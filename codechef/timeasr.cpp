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
  int t;
  cin >> t;
  for (int i=0; i<t; i++) {
    double d;
    cin >> d;
    // 1 / 120000 degree
    int deg = (int)(d * 10000 + 0.5) * 12;
    for (int hour = 0; hour < 12; hour++) {
      for (int minute = 0; minute < 60; minute++) {
        int h_angle = (hour * 60 + minute) * 120000 / 2;
        int m_angle = minute * 120000 * 6;
        int diff = abs(m_angle - h_angle);
        diff = min(diff, 360 * 120000 - diff);
        if (abs(diff - deg) < 1000) {
          printf("%02d:%02d\n", hour, minute);
        }
      }
    }
  }

  return 0;
}

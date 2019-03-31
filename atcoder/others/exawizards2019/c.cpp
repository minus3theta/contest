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
  int n, q;
  cin >> n >> q;
  string s;
  cin >> s;
  vector<pair<char, char>> tds(q);
  FOR(td, tds) {
    string a, b;
    cin >> a >> b;
    td.first = a[0];
    td.second = b[0];
  }
  int l = -1;
  int r = n;
  while (l + 1 < r) {
    int m = (l + r) / 2;
    int x = m;
    FOR(td, tds) {
      char tx = s[x];
      char t = td.first;
      char d = td.second;
      if (tx == t) {
        if (d == 'L') {
          x--;
        } else {
          x++;
        }
      }
      if (x < 0 || x >= n) {
        break;
      }
    }
    if (x < 0) {
      l = m;
    } else {
      r = m;
    }
  }
  int ld = l;
  l = -1;
  r = n;
  while (l + 1 < r) {
    int m = (l + r) / 2;
    int x = m;
    FOR(td, tds) {
      char tx = s[x];
      char t = td.first;
      char d = td.second;
      if (tx == t) {
        if (d == 'L') {
          x--;
        } else {
          x++;
        }
      }
      if (x < 0 || x >= n) {
        break;
      }
    }
    if (x >= n) {
      r = m;
    } else {
      l = m;
    }
  }
  int rd = r;
  cout << rd - ld - 1 << endl;

  return 0;
}

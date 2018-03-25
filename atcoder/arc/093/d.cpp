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
  int a, b;
  cin >> a >> b;
  vector<string> fld(100, string(100, '#'));
  REP(i,50,100) {
    REP(j,0,100) {
      fld[i][j] = '.';
    }
  }
  int i = 0;
  int j = 0;
  for(a--; a > 0; a--) {
    fld[i][j] = '.';
    j += 2;
    if(j >= 100) {
      j = 0;
      i += 2;
    }
  }
  i = 52;
  j = 0;
  for(b--; b > 0; b--) {
    fld[i][j] = '#';
    j += 2;
    if(j >= 100) {
      j = 0;
      i += 2;
    }
  }
  cout << "100 100" << endl;
  FOR(v,fld) {
    cout << v << endl;
  }

  return 0;
}

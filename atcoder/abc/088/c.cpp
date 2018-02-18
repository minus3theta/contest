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
  vector<VI> cs(3, VI(3));
  FOR(c,cs) {
    FOR(x,c) {
      cin >> x;
    }
  }
  bool ans = true;
  REP(i,0,2) {
    REP(j,0,2) {
      ans = ans && cs[i][j] - cs[i][j+1] == cs[i+1][j] - cs[i+1][j+1];
      ans = ans && cs[i][j] - cs[i+1][j] == cs[i][j+1] - cs[i+1][j+1];
    }
  }
  cout << (ans ? "Yes" : "No") << endl;
  
  return 0;
}

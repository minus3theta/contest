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
  int maxabs = -1;
  int argmax = -1;
  REP(i,0,n) {
    if(abs(as[i]) > maxabs) {
      maxabs = abs(as[i]);
      argmax = i;
    }
  }
  cout << n * 2 - 2 << endl;
  REP(i,0,n) {
    if(i != argmax) {
      cout << argmax + 1 << " " << i + 1 << endl;
    }
  }
  if(as[argmax] >= 0) {
    REP(i,1,n) {
      cout << i << " " << i + 1 << endl;
    }
  } else {
    for(int i=n; i>1; i--) {
      cout << i << " " << i - 1 << endl;
    }
  }

  return 0;
}

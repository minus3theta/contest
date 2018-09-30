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

constexpr int M = 33;
constexpr ll D = (1ll << M) - 1;

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  vector<PL> ps(n);
  FOR(p,ps) {
    cin >> p.first >> p.second;
  }
  ll parity = abs(ps[0].first + ps[0].second) % 2;
  FOR(p,ps) {
    if(abs(p.first + p.second) % 2 != parity) {
      cout << -1 << endl;
      return 0;
    }
  }
  if(parity) {
    cout << M << endl;
  } else {
    cout << M + 1 << endl;
    cout << 1 << " ";
  }
  REP(i,0,M) {
    cout << (1ll << i) << " ";
  }
  cout << endl;
  FOR(p,ps) {
    PL delta;
    if(parity) {
      delta = {p.first - D, p.second};
    } else {
      delta = {p.first - D - 1, p.second};
      cout << 'R';
    }
    ll du = (-delta.first + delta.second) / 2;
    ll dv = (-delta.first - delta.second) / 2;
    REP(i,0,M) {
      if((du & 1) == 0) {
        if((dv & 1) == 0) {
          cout << 'R';
        } else {
          cout << 'D';
        }
      } else {
        if((dv & 1) == 0) {
          cout << 'U';
        } else {
          cout << 'L';
        }
      }
      du >>= 1;
      dv >>= 1;
    }
    cout << endl;
  }

  return 0;
}

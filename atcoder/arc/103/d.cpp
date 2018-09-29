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

void solveX(ll src, ll dst, int arm) {
  ll delta = dst - src;
  int r = (arm + delta) / 2;
  REP(i,0,r) {
    cout << 'R';
  }
  REP(i,0,arm-r) {
    cout << 'L';
  }
}

void solveY(ll src, ll dst, int arm) {
  ll delta = dst - src;
  int r = (arm + delta) / 2;
  REP(i,0,r) {
    cout << 'U';
  }
  REP(i,0,arm-r) {
    cout << 'D';
  }
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  vector<PL> ps(n);
  FOR(p,ps) {
    cin >> p.first >> p.second;
    assert(-10 <= p.first && p.first <= 10);
    assert(-10 <= p.second && p.second <= 10);
  }
  ll parity = abs(ps[0].first + ps[0].second) % 2;
  FOR(p,ps) {
    if(abs(p.first + p.second) % 2 != parity) {
      cout << -1 << endl;
      return 0;
    }
  }
  cout << 40 << endl;
  if(parity) {
    cout << 2;
    REP(i,0,39) {
      cout << " " << 1;
    }
    cout << endl;
    FOR(p,ps) {
      cout << 'R';
      if(p.first % 2 == 0) {
        solveX(2, p.first, 20);
        solveY(0, p.second, 19);
      } else {
        solveX(2, p.first, 19);
        solveY(0, p.second, 20);
      }
      cout << endl;
    }
  } else {
    cout << 1;
    REP(i,0,39) {
      cout << " " << 1;
    }
    cout << endl;
    FOR(p,ps) {
      if(p.first % 2 == 0) {
        solveX(0, p.first, 20);
        solveY(0, p.second, 20);
      } else {
        solveX(0, p.first, 19);
        solveY(0, p.second, 21);
      }
      cout << endl;
    }
  }

  return 0;
}

#include <cstdio>
#include <climits>
#include <cassert>
#include <cmath>
#include <iostream>
#include <string>
#include <algorithm>
#include <numeric>
#include <functional>
#include <list>
#include <map>
#include <queue>
#include <stack>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

// 2-dimensional BIT
// 1-indexed
template <class T, class Op, T unit>
struct Bit2 {
  int m, n;
  vector<vector<T> > dat;
  Op op;
  Bit2(int m, int n) : m(m), n(n), dat(m + 1, vector<T>(n + 1, unit)) {
  }
  void operate(int k0, int l0, T a) {
    for(int k=k0; k <= m; k += k&-k) {
      for(int l=l0; l <= n; l += l&-l) {
        dat[k][l] = op(dat[k][l], a);
      }
    }
  }
  T accum(int k0, int l0) {
    T sum = unit;
    for(int k=k0; k > 0; k -= k&-k) {
      for(int l=l0; l > 0; l -= l&-l) {
        sum = op(sum, dat[k][l]);
      }
    }
    return sum;
  }
};

int main() {
  ios::sync_with_stdio(false);
  int X;
  cin >> X;
  REP(i,0,X) {
    int N, T;
    cin >> N >> T;
    Bit2<int, plus<int>, 0> bit(N+1, N+1);
    REP(j,0,T) {
      string query;
      cin >> query;
      if(query == "C") {
        int x1, y1, x2, y2;
        cin >> x1 >> y1 >> x2 >> y2;
        x2++;
        y2++;
        bit.operate(x1,y1,1);
        bit.operate(x1,y2,-1);
        bit.operate(x2,y1,-1);
        bit.operate(x2,y2,1);
      } else {
        int x, y;
        cin >> x >> y;
        cout << bit.accum(x,y) % 2 << endl;
      }
    }
    cout << endl;
  }
  return 0;
}

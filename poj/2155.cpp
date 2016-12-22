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

// 1-indexed
struct Bit {
  int n;
  vector<VI> dat;
  Bit(int n) : n(n), dat(n + 1, VI(n + 1, 0)) {
  }
  void operate(int k, int l0, int a) {
    while(k <= n) {
      int l = l0;
      while(l <= n) {
        dat[k][l] += a;
        l += l & -l;
      }
      k += k & -k;
    }
  }
  int accum(int k, int l0) {
    int sum = 0;
    while(k > 0) {
      int l = l0;
      while(l > 0) {
        sum += dat[k][l];
        l -= l & -l;
      }
      k -= k & -k;
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
    Bit bit(N+1);
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

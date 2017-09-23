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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

int main() {
  ios::sync_with_stdio(false);
  int H, W, d;
  cin >> H >> W >> d;
  vector<string> fld(H);
  REP(i,0,H) {
    REP(j,0,W) {
      int p = i + j;
      int q = i - j;
      int x = p % (2 * d);
      int y = (q % (2 * d) + 2 * d) % (2 * d);
      if(x < d) {
        if(y < d) {
          fld[i].push_back('R');
        } else {
          fld[i].push_back('Y');
        }
      } else {
        if(y < d) {
          fld[i].push_back('G');
        } else {
          fld[i].push_back('B');
        }
      }
    }
  }
  FOR(s,fld) {
    cout << s << endl;
  }
  return 0;
}

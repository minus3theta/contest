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
  int H, W, K;
  cin >> H >> W >> K;
  vector<string> field(H);
  FOR(s,field) {
    cin >> s;
  }
  cout << H * (W/K) << endl;
  REP(i,0,H) {
    REP(j,0,W/K) {
      REP(k,0,K) {
        cout << i + 1 << " " << j*K + k + 1 << endl;
      }
    }
  }
  return 0;
}

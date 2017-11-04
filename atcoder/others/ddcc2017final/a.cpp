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

bool include(int x, int y, int r) {
  return (x - r) * (x - r) + (y - r) * (y - r) <= r * r;
}

int solve(int K, int S) {
  int c = 0;
  int R = S / 2;
  REP(i,0,S/K) {
    REP(j,0,S/K) {
      bool inc = true;
      REP(p,0,2) {
        REP(q,0,2) {
          if(!include((i+p) * K, (j+q) * K, R)) {
            inc = false;
          }
        }
      }
      if(inc) {
        c++;
      }
    }
  }
  return c;
}

int main() {
  ios::sync_with_stdio(false);
  int K;
  cin >> K;
  cout << solve(K, 200) << " " << solve(K, 300) << endl;
  
  return 0;
}

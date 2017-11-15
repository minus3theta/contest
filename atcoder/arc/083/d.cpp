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
  int N;
  cin >> N;
  vector<VL> as(N, VL(N));
  FOR(v,as) {
    FOR(x,v) {
      cin >> x;
    }
  }
  vector<VL> bs = as;
  REP(i,0,N) {
    REP(j,0,N) {
      REP(k,0,N) {
        bs[j][k] = min(bs[j][k], bs[j][i] + bs[i][k]);
      }
    }
  }
  REP(i,0,N) {
    REP(j,0,N) {
      if(as[i][j] > bs[i][j]) {
        cout << -1 << endl;
        return 0;
      }
    }
  }
  ll sum = 0;
  REP(i,0,N) {
    REP(j,i+1,N) {
      bool use = true;
      REP(k,0,N) {
        if(k != i && k != j && bs[i][j] == bs[i][k] + bs[k][j]) {
          use = false;
          break;
        }
      }
      if(use) {
        sum += bs[i][j];
      }
    }
  }
  cout << sum << endl;

  return 0;
}

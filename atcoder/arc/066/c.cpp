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

const ll MOD = 1e9 + 7;

int main() {
  ios::sync_with_stdio(false);
  ll N;
  cin >> N;
  VI as(N, 0);
  REP(i,0,N) {
    int a;
    cin >> a;
    as[a]++;
  }
  bool pos = true;
  if(N % 2 == 0) {
    for(int i=1; i<N; i+=2) {
      if(as[i] != 2) {
        pos = false;
        break;
      }
    }
  } else {
    if(as[0] != 1) {
      pos = false;
    } else {
      for(int i=2; i<N; i+=2) {
        if(as[i] != 2) {
          pos = false;
          break;
        }
      }
    }
  }
  if(!pos) {
    cout << 0 << endl;
    return 0;
  }
  ll ans = 1;
  if(N % 2 == 0) {
    for(int i=1; i<N; i+=2) {
      ans = (ans * 2) % MOD;
    }
  } else {
    for(int i=2; i<N; i+=2) {
      ans = (ans * 2) % MOD;
    }
  }
  cout << ans << endl;

  return 0;
}

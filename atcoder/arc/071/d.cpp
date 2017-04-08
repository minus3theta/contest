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

const ll MOD = 1e9+7;

ll neg(ll x) {
  return MOD - x;
}

int main() {
  ios::sync_with_stdio(false);
  VI nm(2);
  cin >> nm[0] >> nm[1];
  vector<VL> ls(2);
  REP(i,0,2) {
    REP(j,0,nm[i]) {
      ll x;
      cin >> x;
      ls[i].push_back(x);
    }
  }
  VL sum(2);
  REP(i,0,2) {
    sum[i] = ls[i][nm[i]-1];
  }
  VL res(2, 0);
  REP(i,0,2) {
    for(int j=nm[i]-2; j>=0; j--) {
      res[i] += (sum[i] + neg((nm[i] - j - 1) * ls[i][j]) % MOD) % MOD;
      res[i] %= MOD;
      sum[i] = (sum[i] + ls[i][j]) % MOD;
    }
  }
  cout << ((res[0] * res[1]) % MOD + MOD) % MOD << endl;
  return 0;
}

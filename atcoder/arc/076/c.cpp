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

const ll mod = 1e9+7;

ll fact(ll x) {
  ll ans = 1;
  REP(i,1,x+1) {
    ans = (ans * i) % mod;
  }
  return ans;
}

int main() {
  ios::sync_with_stdio(false);
  ll N, M;
  cin >> N >> M;
  ll ans;
  if(abs(N - M) > 1) {
    ans = 0;
  } else if(N == M) {
    ans = (fact(N) * fact(M) * 2) % mod;
  } else {
    ans = (fact(N) * fact(M)) % mod;
  }
  cout << ans << endl;
  return 0;
}

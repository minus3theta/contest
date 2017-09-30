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
  ll N, K;
  cin >> N >> K;
  vector<PL> ns(N);
  FOR(n,ns) {
    cin >> n.first >> n.second;
  }
  ll ans = 0;
  FOR(n,ns) {
    if((n.first | K) == K) {
      ans += n.second;
    }
  }
  for(ll p=1; p <= K; p <<= 1) {
    if(!(p & K)) {
      continue;
    }
    ll k = (K ^ p) | (p - 1);
    ll v = 0;
    FOR(n,ns) {
      if((n.first | k) == k) {
        v += n.second;
      }
    }
    ans = max(ans, v);
  }
  cout << ans << endl;
  return 0;
}

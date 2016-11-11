#include <cstdio>
#include <climits>
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

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

ll pow(ll a, ll b, ll mod) {
  if(b == 0) return 1;
  ll x = pow(a, b/2, mod);
  x = (x * x) % mod;
  if(b % 2) {
    return (x * a) % mod;
  }
  return x % mod;
}

int main() {
  ll Z;
  cin >> Z;
  REP(i,0,Z) {
    ll M, H;
    cin >> M >> H;
    ll ans = 0;
    REP(j,0,H) {
      ll a, b;
      cin >> a >> b;
      ans = (ans + pow(a, b, M)) % M;
    }
    cout << ans << endl;
  }
  return 0;
}

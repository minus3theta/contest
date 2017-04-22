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

ll solve(VL &as, bool positive) {
  ll ans = 0;
  ll sum = 0;
  FOR(a,as) {
    ll nsum = sum + a;
    if(positive) {
      if(nsum <= 0) {
        ans += 1 - nsum;
        nsum = 1;
      }
    } else {
      if(nsum >= 0) {
        ans += 1 + nsum;
        nsum = -1;
      }
    }
    sum = nsum;
    positive = !positive;
  }
  return ans;
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VL as(n);
  FOR(a,as) {
    cin >> a;
  }
  cout << min(solve(as, true), solve(as, false)) << endl;
  return 0;
}

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
  VL as(3 * N);
  FOR(a, as) {
    cin >> a;
  }
  sort(as.rbegin(), as.rend());
  ll ans = 0;
  REP(i,0,N) {
    ans += as[i * 2 + 1];
  }
  cout << ans << endl;
  return 0;
}

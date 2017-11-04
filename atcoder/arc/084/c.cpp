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
  vector<VL> parts(3, VL(N));
  FOR(v, parts) {
    FOR(x, v) {
      cin >> x;
    }
    sort(v.begin(), v.end());
  }
  VL smaller(N);
  VL greater(N);
  int a = 0;
  REP(i,0,N) {
    ll b = parts[1][i];
    while(a < N && parts[0][a] < b) {
      a++;
    }
    smaller[i] = a;
  }
  int c = 0;
  for(int i = N-1; i>= 0; i--) {
    ll b = parts[1][i];
    while(c < N && parts[2][N-c-1] > b) {
      c++;
    }
    greater[i] = c;
  }
  ll ans = 0;
  REP(i,0,N) {
    ans += smaller[i] * greater[i];
  }
  cout << ans << endl;

  return 0;
}

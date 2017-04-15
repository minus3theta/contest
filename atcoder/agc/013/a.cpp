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
  VL as(N);
  FOR(a,as) {
    cin >> a;
  }
  int ans = 1;
  bool up = false;
  bool down = false;
  ll cur = as[0];
  REP(i,1,N) {
    ll a = as[i];
    if(!up && !down) {
      if(a == cur) continue;
      if(a > cur) {
        up = true;
      } else {
        down = true;
      }
      cur = a;
    } else if(up) {
      if(a < cur) {
        ans++;
        up = down = false;
      }
      cur = a;
    } else {
      if(a > cur) {
        ans++;
        up = down = false;
      }
      cur = a;
    }
  }
  cout << ans << endl;
  return 0;
}

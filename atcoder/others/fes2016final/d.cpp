#include <cstdio>
#include <climits>
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

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

int main() {
  int N, M;
  cin >> N >> M;
  VI vx(N);
  VI mod(M,0);
  VI pr(M,0);
  for(auto &x : vx) {
    cin >> x;
  }
  sort(vx.begin(), vx.end());
  int prvx = -1;
  for(auto &x : vx) {
    mod[x % M]++;
    if(prvx == x) {
      pr[x % M]++;
      prvx = -1;
    } else {
      prvx = x;
    }
  }
  ll ans = 0;
  ans += mod[0] / 2;
  if(M % 2 == 0) {
    ans += mod[M/2] / 2;
  }
  REP(i,1,(M+1)/2) {
    int diff = mod[i] - mod[M-i];
    if(diff >= 2) {
      int p = min(diff / 2, pr[i]);
      ans += p + mod[M-i];
    } else if (diff <= -2) {
      int p = min((-diff) / 2, pr[M-i]);
      ans += p + mod[i];
    } else {
      ans += min(mod[i], mod[M-i]);
    }
  }
  cout << ans << endl;
  return 0;
}

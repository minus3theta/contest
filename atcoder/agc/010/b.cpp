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
  ll N;
  cin >> N;
  ll series = N * (N+1) / 2;
  VL as(N);
  ll sum = 0;
  FOR(a,as) {
    cin >> a;
    sum += a;
  }
  if(sum % series != 0) {
    cout << "NO" << endl;
    return 0;
  }
  ll times = sum / series;
  ll left_times = times;
  REP(i,0,N) {
    ll diff = as[i] - as[(i+1)%N] + times;
    if(diff < 0 || diff % N != 0) {
      cout << "NO" << endl;
      return 0;
    }
    left_times -= diff / N;
  }
  assert(left_times == 0);
  cout << "YES" << endl;
  return 0;
}

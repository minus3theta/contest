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

const ll N = 50;

int main() {
  ios::sync_with_stdio(false);
  ll K;
  cin >> K;
  ll times = K / N;
  ll base = N - 1 + times - K % N;
  VL ans(N, base);
  REP(i,0,N) {
    if(i < K % N) {
      ans[i] += N;
    // } else {
    //   ans[i] -= K % N;
    }
  }
  cout << N << endl;
  FOR(a,ans) {
    cout << a << " ";
  }
  cout << endl;
  return 0;
}

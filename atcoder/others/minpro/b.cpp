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
  VL A(N);
  FOR(a,A) {
    cin >> a;
  }
  sort(A.begin(), A.end());
  ll sum = K * (K - 1) / 2;
  REP(i,0,K) {
    sum += A[i];
  }
  cout << sum << endl;
  return 0;
}

#include <cstdio>
#include <climits>
#include <cassert>
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

ll count_le(const VL &A, const VL &B, ll val) {
  ll n = 0;
  FOR(a,A) {
    auto i = upper_bound(B.begin(), B.end(), val / a);
    n += i - B.begin();
  }
  return n;
}

int main() {
  ll N, K;
  cin >> N >> K;
  VL A(N), B(N);
  FOR(a,A) {
    cin >> a;
  }
  FOR(b,B) {
    cin >> b;
  }
  sort(A.begin(),A.end());
  sort(B.begin(),B.end());
  ll l = -1;
  ll r = 1e18;
  while(l + 1 != r) {
    ll m = (l + r) / 2;
    if(count_le(A, B, m) < K) {
      l = m;
    } else {
      r = m;
    }
  }
  cout << r << endl;
  return 0;
}

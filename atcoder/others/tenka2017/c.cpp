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

bool query(ll N, ll h, ll n, ll w) {
  return 4 * h * n * w <= N * (n * w + w * h + h * n);
}

ll get_w(ll N, ll h, ll n) {
  ll l = n;
  ll r = 3501;
  while (l + 1 != r) {
    ll m = (l + r) / 2;
    if(query(N, h, n, m)) {
      l = m;
    } else {
      r = m;
    }
  }
  if(4 * h * n * l == N * (n * l + l * h + h * n)) {
    return l;
  }
  return -1;
}

void solve(ll N) {
  for(ll h=1; h<=3500; h++) {
    for(ll n=h; n<=3500; n++){
      ll w = get_w(N, h, n);
      if(w != -1) {
        cout << h << " " << n << " " << w << endl;
        return;
      }
    }
  }
}

int main() {
  ios::sync_with_stdio(false);
  ll N;
  cin >> N;
  solve(N);
  return 0;
}

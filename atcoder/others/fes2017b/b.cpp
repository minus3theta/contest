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

bool solve(const VL &ds, const VL &ts) {
  int i = 0;
  FOR(t,ts) {
    while(ds[i] < t) {
      i++;
    }
    if(ds[i] == t) {
      i++;
    } else {
      return false;
    }
  }
  return true;
}

int main() {
  ios::sync_with_stdio(false);
  int N, M;
  cin >> N;
  VL ds(N);
  FOR(d,ds) {
    cin >> d;
  }
  cin >> M;
  VL ts(M);
  FOR(t,ts) {
    cin >> t;
  }
  sort(ds.begin(), ds.end());
  sort(ts.begin(), ts.end());
  cout << (solve(ds, ts) ? "YES" : "NO") << endl;
  
  return 0;
}

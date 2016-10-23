#include <cstdio>
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

const int MOD = 1e9 + 7;

int main() {
  int n;
  cin >> n;
  vector<int> t(n);
  vector<int> a(n);
  REP(i,0,n) {
    cin >> t[i];
  }
  REP(i,0,n) {
    cin >> a[i];
  }
  vector<pair<int,int>> vt(n);
  vector<pair<int,int>> va(n);
  REP(i,0,n) {
    int pt = i == 0 ? 1 : t[i-1];
    vt[i].first = t[i];
    vt[i].second = pt == t[i] ? 1 : t[i];
  }
  for(int i=n-1; i>=0; i--) {
    int pa = i == n-1 ? 1 : a[i+1];
    va[i].first = a[i];
    va[i].second = pa == a[i] ? 1 : a[i];
  }
  ll c = 1;
  REP(i,0,n) {
    int u = min(vt[i].first, va[i].first);
    int l = max(vt[i].second, va[i].second);
    int x = u - l + 1;
    if(x < 1) {
      c = 0;
      break;
    }
    c = (c * (x % MOD)) % MOD;
  }
  cout << c << endl;
  return 0;
}

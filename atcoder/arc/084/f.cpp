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
using VB = vector<bool>;

const ll mod = 998244353L;

void write(const VB &x) {
  for(bool b: x) {
    cout << b;
  }
  cout << endl;
}

void addAssign(VB &x, const VB &y, int offset) {
  int n = x.size();
  REP(i,offset,n) {
    x[i] = x[i] != y[i - offset];
  }
  while(!x.empty()) {
    if(x.back()) break;
    x.pop_back();
  }
}

void gcd(VB &x, VB &y) {
  if(x.size() < y.size()) {
    x.swap(y);
  }
  while(y.size() > 0) {
    addAssign(x, y, x.size() - y.size());
    if(x.size() < y.size()) {
      x.swap(y);
    }
  }
}

ll solve(const VB &x, const VB &g) {
  int xl = x.size();
  int gl = g.size();
  if(xl < gl) {
    return 1;
  }
  ll ans = 0;
  for(int i=xl-1; i>=gl-1; i--) {
    ans = (ans * 2) % mod;
    if(x[i]) {
      ans = (ans + 1) % mod;
    }
  }
  VB y(xl);
  copy(g.begin(), g.end(), y.begin() + (xl - gl));
  for(int i=xl-1; i>=gl-1; i--) {
    if(x[i] != y[i]) {
      addAssign(y, g, i - gl + 1);
    }
  }
  ans++;
  for(int i=gl-1; i>=0; i--) {
    if(x[i] < y[i]) {
      ans--;
      break;
    }
    if(x[i] > y[i]) {
      break;
    }
  }
  return ans;
}

int main() {
  ios::sync_with_stdio(false);
  int N;
  cin >> N;
  string str;
  cin >> str;
  VB x;
  transform(str.rbegin(), str.rend(), back_inserter(x), [](char c){return c == '1';});
  vector<VB> as(N);
  FOR(a,as) {
    cin >> str;
    transform(str.rbegin(), str.rend(), back_inserter(a), [](char c){return c == '1';});
  }
  VB g;
  FOR(a,as) {
    gcd(g, a);
  }
  cout << solve(x, g) << endl;

  return 0;
}

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

int substrlen(const string &s, const string &t) {
  unsigned i = 0;
  while(i < s.size() && i < t.size() && s[i] == t[i]) {
    i++;
  }
  return i;
}

int main() {
  ios::sync_with_stdio(false);
  ll N, K;
  cin >> N >> K;
  VL A(K);
  vector<pair<string,bool>> S(N, {"", false});
  vector<string> T;
  FOR(a,A) {
    cin >> a;
  }
  FOR(s,S) {
    cin >> s.first;
  }
  FOR(a,A) {
    S[a-1].second = true;
  }
  int r = A[0] - 1;
  int mina = INT_MAX;
  int maxb = -1;
  REP(i,0,N) {
    if(i == r) continue;
    int l = substrlen(S[r].first, S[i].first);
    if(S[i].second) {
      mina = min(mina, l);
    } else {
      maxb = max(maxb, l);
    }
  }
  // cout << maxb << " , " << mina << endl;
  if(maxb < mina) {
    cout << S[r].first.substr(0, maxb+1) << endl;
  } else {
    cout << -1 << endl;
  }
  return 0;
}

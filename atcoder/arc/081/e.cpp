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

const int M = 26;

int main() {
  ios::sync_with_stdio(false);
  string a;
  cin >> a;
  int n = a.size();
  VI sep;
  vector<bool> app(M, false);
  int appcount = 0;
  vector<VI> pos(M);
  for(int i = n-1; i >= 0; i--) {
    if(!app[a[i]-'a']) {
      app[a[i]-'a'] = true;
      appcount++;
      if(appcount == M) {
        sep.push_back(i);
        app.assign(M, false);
        appcount = 0;
      }
    }
  }
  if(sep.empty()) {
    REP(i,0,M) {
      if(!app[i]) {
        cout << (char)('a'+i) << endl;
        return 0;
      }
    }
  }
  reverse(sep.begin(), sep.end());
  sep.push_back(n);
  REP(i,0,n) {
    pos[a[i]-'a'].push_back(i);
  }
  int len = sep.size();
  int used = -1;
  string ans;
  REP(i,0,len) {
    REP(c,0,M) {
      auto cp = lower_bound(pos[c].begin(), pos[c].end(), used);
      if(cp == pos[c].end() || *cp >= sep[i]) {
        ans.push_back('a' + c);
        used = *cp + 1;
        break;
      }
    }
  }
  cout << ans << endl;
  
  return 0;
}

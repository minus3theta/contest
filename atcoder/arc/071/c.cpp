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
  int n;
  cin >> n;
  vector<string> ss(n);
  FOR(s,ss) {
    cin >> s;
  }
  VI minimum(26, INT_MAX);
  FOR(s,ss) {
    VI count(26, 0);
    FOR(c,s) {
      count[c-'a']++;
    }
    REP(i,0,26) {
      minimum[i] = min(minimum[i], count[i]);
    }
  }
  string ans = "";
  REP(i,0,26) {
    REP(j,0,minimum[i]) {
      ans.push_back(i + 'a');
    }
  }
  cout << ans << endl;
  return 0;
}

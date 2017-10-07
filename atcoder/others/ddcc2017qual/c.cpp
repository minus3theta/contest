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
  int N;
  ll C;
  cin >> N >> C;
  VL ls(N);
  FOR(l,ls) {
    cin >> l;
  }
  sort(ls.begin(), ls.end());
  ll ans = 0;
  int l = 0;
  int r = N-1;
  while(l <= r) {
    if(ls[l] + ls[r] + 1 <= C) {
      l++;
      r--;
      ans++;
    } else {
      r--;
      ans++;
    }
  }
  cout << ans << endl;
  
  return 0;
}

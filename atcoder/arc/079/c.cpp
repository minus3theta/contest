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
  int N, M;
  cin >> N >> M;
  vector<bool> s(N+1, false);
  vector<bool> e(N+1, false);
  REP(i,0,M) {
    int a, b;
    cin >> a >> b;
    if(a == 1) {
      s[b] = true;
    } else if(a == N) {
      e[b] = true;
    }
    if(b == 1) {
      s[a] = true;
    } else if(b == N) {
      e[a] = true;
    }
  }
  bool ans = false;
  REP(i,2,N) {
    if(s[i] && e[i]) {
      ans = true;
      break;
    }
  }
  cout << (ans ? "POSSIBLE" : "IMPOSSIBLE") << endl;
  return 0;
}

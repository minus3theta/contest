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
  ll Z, W;
  cin >> N >> Z >> W;
  VL as(N);
  FOR(a,as) {
    cin >> a;
  }
  if(N == 1) {
    cout << llabs(as[0] - W) << endl;
  } else {
    cout << max(llabs(as[N-2]-as[N-1]), llabs(as[N-1]-W)) << endl;
  }

  return 0;
}

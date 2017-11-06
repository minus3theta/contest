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

VI solve(int K, int N) {
  if(K % 2 == 0) {
    VI ans(N, K);
    ans[0] = K / 2;
    return ans;
  }
  VI ans(N, (K+1)/2);
  REP(i,0,N/2) {
    int &x = ans.back();
    if(x == 1) {
      ans.pop_back();
    } else {
      x--;
      while((int)ans.size() < N) {
        ans.push_back(K);
      }
    }
  }
  return ans;
}

int main() {
  ios::sync_with_stdio(false);
  int K, N;
  cin >> K >> N;
  VI ans = solve(K, N);
  FOR(a,ans) {
    cout << a << " ";
  }
  cout << endl;

  return 0;
}

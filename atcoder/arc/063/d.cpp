#include <cstdio>
#include <climits>
#include <iostream>
#include <string>
#include <algorithm>
#include <numeric>
#include <functional>
#include <list>
#include <map>
#include <queue>
#include <stack>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long ll;

const int INF = INT_MAX;

int main() {
  int N, T;
  cin >> N >> T;
  vector<int> A(N);
  REP(i,0,N) {
    cin >> A[i];
  }
  int mina = INF;
  int gap = -1;
  for(auto &x: A) {
    mina = min(mina, x);
    gap = max(gap, x - mina);
  }
  int ans = 0;
  mina = INF;
  for(auto &x: A) {
    mina = min(mina, x);
    if(x - mina == gap) {
      ans++;
    }
  }
  cout << ans << endl;
  return 0;
}

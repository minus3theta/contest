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

int main() {
  int k, t;
  cin >> k >> t;
  vector<int> v(t);
  int m = 0;
  REP(i,0,t) {
    cin >> v[i];
    m = max(m, v[i]);
  }
  // int x = (k + 1) / 2;
  // cout << (t == 1 ? k - 1 : max(0, m - x)) << endl;
  cout << max(0, k - 2 * (k - m) - 1) << endl;
  return 0;
}

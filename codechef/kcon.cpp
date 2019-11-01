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
#include <iterator>
#include <regex>

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

ll max_range(const vector<ll> &a) {
  int n = a.size();
  vector<ll> sum(n+1, 0);
  // sum[i] = sum of a[0, i)
  for (int i=0; i<n; i++) {
    sum[i+1] = sum[i] + a[i];
  }
  ll min_sum = 0;
  ll ret = -(1ll << 60);
  for (int i=1; i<=n; i++) {
    ret = max(ret, sum[i] - min_sum);
    min_sum = min(min_sum, sum[i]);
  }
  return ret;
}

ll solve(vector<ll> &a, int k) {
  ll total = accumulate(a.begin(), a.end(), 0ll);
  if (k >= 2) {
    int n = a.size();
    for (int i=0; i<n; i++) {
      a.push_back(a[i]);
    }
  }
  ll base = max_range(a);
  if (total > 0 && k >= 3) {
    return base + total * (k - 2);
  }
  return base;
}

int main() {
  ios::sync_with_stdio(false);
  int t;
  cin >> t;
  for (int i=0; i<t; i++) {
    int n, k;
    cin >> n >> k;
    vector<ll> a(n);
    for (ll &x: a) {
      cin >> x;
    }
    cout << solve(a, k) << endl;
  }

  return 0;
}

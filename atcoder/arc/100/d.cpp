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

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

template <class T, template <class, class> class C, class charT = char>
void vdump(const C<T, allocator<T>> &v, const charT* delimiter = ", ",
           ostream &stream = cout) {
  copy(v.begin(), v.end(), ostream_iterator<T>(stream, delimiter));
  stream << endl;
}

PL min_partition(const VL &sum, int left, int right, ll val) {
  auto lb = lower_bound(sum.begin() + left, sum.begin() + right, val);
  int lb1 = lb - sum.begin();
  ll large = 1e18;
  if (left < lb1 && lb1 < right) {
    large = min(large, sum[lb1] - sum[left]);
  }
  int lb0 = lb1 - 1;
  if (left < lb0 && lb0 < right) {
    large = min(large, sum[right] - sum[lb0]);
  }
  return {sum[right] - sum[left] - large, large};
}

ll solve(const VL &sum, int mid) {
  int n = sum.size();
  ll left_sum = sum[mid];
  PL left_mm = min_partition(sum, 0, mid, (left_sum) / 2);
  PL right_mm = min_partition(sum, mid, n-1, (sum[n-1] + sum[mid]) / 2);
  return max(left_mm.second, right_mm.second) - min(left_mm.first, right_mm.first);
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VL as(n);
  FOR(a,as) {
    cin >> a;
  }
  VL sum(n+1);
  REP(i,0,n) {
    sum[i+1] = sum[i] + as[i];
  }
  ll ans = 1e18;
  REP(mid,2,n-1) {
    ans = min(ans, solve(sum, mid));
  }
  cout << ans << endl;

  return 0;
}

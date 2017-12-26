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

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VL as(3 * n);
  FOR(a,as) {
    cin >> a;
  }
  VL head(n+1);
  ll head_sum = accumulate(as.begin(), as.begin() + n, 0ll);
  priority_queue<ll, VL, greater<ll>> head_que(as.begin(), as.begin() + n);
  REP(k,0,n+1) {
    head[k] = head_sum;
    ll x = as[n+k];
    head_sum += x;
    head_que.push(x);
    head_sum -= head_que.top();
    head_que.pop();
  }
  VL tail(n+1);
  ll tail_sum = accumulate(as.rbegin(), as.rbegin() + n, 0ll);
  priority_queue<ll> tail_que(as.rbegin(), as.rbegin() + n);
  for(int k=n; k>=0; k--) {
    tail[k] = tail_sum;
    ll x = as[n+k-1];
    tail_sum += x;
    tail_que.push(x);
    tail_sum -= tail_que.top();
    tail_que.pop();
  }
  ll ans = LLONG_MIN;
  REP(i,0,n+1) {
    ans = max(ans, head[i] - tail[i]);
  }
  cout << ans << endl;
  
  return 0;
}

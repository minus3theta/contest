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

int pop(const VL &bs, int bit) {
  int ret = 0;
  FOR(b,bs) {
    if(b >> bit & 1) {
      ret++;
    }
  }
  return ret;
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  int k;
  cin >> n >> k;
  VL as(n);
  FOR(a,as) {
    cin >> a;
  }
  VL sum(n+1, 0);
  REP(i,0,n) {
    sum[i+1] = sum[i] + as[i];
  }
  VL bs;
  REP(i,0,n) {
    REP(j,i+1,n+1) {
      bs.push_back(sum[j] - sum[i]);
    }
  }
  int bit = 40;
  while(pop(bs, bit) < k) {
    bit--;
  }
  ll ans = 0;
  for(int i=bit; i >= 0; i--) {
    VL can;
    FOR(b,bs) {
      if(b >> i & 1) {
        can.push_back(b);
      }
    }
    if((int)can.size() >= k) {
      ans |= 1ll << i;
      bs = move(can);
    } else {
      continue;
    }
  }
  cout << ans << endl;

  return 0;
}

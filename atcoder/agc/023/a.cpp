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
  VL as(n);
  FOR(a,as) {
    cin >> a;
  }
  VL sum(n+1, 0);
  REP(i,0,n) {
    sum[i+1] = sum[i] + as[i];
  }
  sort(sum.begin(), sum.end());
  ll count = 0;
  ll same = 1;
  ll prev = 1e15;
  REP(i,0,n+1) {
    if(sum[i] == prev) {
      same++;
    } else {
      count += same * (same - 1) / 2;
      same = 1;
      prev = sum[i];
    }
  }
  count += same * (same - 1) / 2;
  cout << count << endl;
  
  return 0;
}

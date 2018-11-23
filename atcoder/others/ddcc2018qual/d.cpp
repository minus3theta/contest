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

constexpr ll target[] = {7, 11, 13, 16, 17, 19, 23, 25, 27, 29};

ll digitsum(ll n, ll base) {
  ll ans = 0;
  while(n > 0) {
    ans += n % base;
    n /= base;
  }
  return ans;
}

int main() {
  ios::sync_with_stdio(false);
  VL as(29);
  FOR(a,as) {
    cin >> a;
  }
  ll divisor = 1;
  ll reminder = 0;
  FOR(t, target) {
    ll b = as[t-1];
    while(reminder % t != b % t) {
      reminder += divisor;
    }
    divisor *= t;
  }
  bool valid = reminder <= 1e12;
  REP(i,0,29) {
    if(digitsum(reminder, i + 2) != as[i]) {
      valid = false;
      break;
    }
  }
  if(valid) {
    cout << reminder << endl;
  } else {
    cout << "invalid" << endl;
  }

  return 0;
}

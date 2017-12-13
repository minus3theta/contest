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

ll gcd(ll a, ll b) {
  if(b == 0) {
    return a;
  }
  return gcd(b, a % b);
}

ll count(ll a, ll b, VL used) {
  if(a > b) {
    return 1;
  }
  ll x = count(a + 1, b, used);
  if(all_of(used.begin(), used.end(), [=](ll p){return gcd(p, a) == 1;})) {
    used.push_back(a);
    x += count(a + 1, b, used);
  }
  return x;
}

int main() {
  ios::sync_with_stdio(false);
  ll a, b;
  cin >> a >> b;
  cout << count(a, b, VL()) << endl;

  return 0;
}

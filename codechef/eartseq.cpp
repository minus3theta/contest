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

void solve(int n) {
  if (n == 5) {
    printf("%d %d %d %d %d\n", 2 * 3, 3 * 5, 5 * 7, 7 * 11, 11 * 2);
    return;
  }
  ll mult = 1;
  while (n > 0) {
    if (n % 3 != 0) {
      printf("%lld ", 2 * 3 * mult);
      mult += 2 * 3 * 5 * 7;
      printf("%lld ", 3 * 5 * mult);
      mult += 2 * 3 * 5 * 7;
      printf("%lld ", 5 * 7 * mult);
      mult += 2 * 3 * 5 * 7;
      printf("%lld ", 7 * 2 * mult);
      mult += 2 * 3 * 5 * 7;
      n -= 4;
    } else {
      printf("%lld ", 2 * 3 * mult);
      mult += 2 * 3 * 5 * 7;
      printf("%lld ", 3 * 5 * mult);
      mult += 2 * 3 * 5 * 7;
      printf("%lld ", 5 * 2 * mult);
      mult += 2 * 3 * 5 * 7;
      n -= 3;
    }
  }
  printf("\n");
}

int main() {
  ios::sync_with_stdio(false);
  int t;
  scanf("%d", &t);
  REP(i,0,t) {
    int n;
    scanf("%d", &n);
    solve(n);
  }

  return 0;
}

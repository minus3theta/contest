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

bool same_line(PL p1, PL p2) {
  if (p1.first == p2.first || p1.second == p2.second) {
    return true;
  }
  ll dx = p1.first - p2.first;
  ll dy = p1.second - p2.second;
  return dx == dy || -dx == dy;
}

const int vx[] = {1, 1, 0, -1};
const int vy[] = {0, 1, 1, 1};

int solve(PL p1, PL p2, PL target) {
  if (p1 == target || p2 == target) {
    return 0;
  }
  if (same_line(p1, target) && same_line(p2, target)) {
    return 1;
  }
  if (same_line(p1, target) || same_line(p2, target)) {
    return 2;
  }
  if (same_line(p1, p2)) {
    return 3;
  }
  REP(i,0,4) {
    REP(j,0,4) {
      if (i == j) continue;
      // solve p1 + k v[i] = p2 - l v[j]
      int det = vx[i] * vy[j] - vx[j] * vy[i];
      ll dx = p2.first - p1.first;
      ll dy = p2.second - p1.second;
      ll k0 = vy[j] * dx - vx[j] * dy;
      if (k0 % det != 0) continue;
      ll k = k0 / det;
      PL q = make_pair(p1.first + k * vx[i], p1.second + k * vy[i]);
      if (same_line(q, target)) {
        return 3;
      }
    }
  }
  return 4;
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  REP(i,0,n) {
    PL p1, p2, target;
    cin >> p1.first >> p1.second >> p2.first >> p2.second >> target.first >> target.second;
    cout << solve(p1, p2, target) << endl;
  }

  return 0;
}

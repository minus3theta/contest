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
  int h, w, k;
  cin >> h >> w >> k;
  vector<vector<bool>> black(h, vector<bool>(w, true));
  REP(i,0,k) {
    int a, b;
    cin >> a >> b;
    black[a-1][b-1] = false;
  }
  vector<VI> up(h, VI(w, 0));
  vector<VI> down(h, VI(w, 0));
  REP (y,0,h) {
    REP(x,0,w) {
      if (! black[y][x]) {
        continue;
      }
      if (y == 0) {
        up[y][x] = 1;
      } else {
        up[y][x] = up[y-1][x] + 1;
      }
    }
  }
  for (int y=h-1; y>=0; y--) {
    REP(x,0,w) {
      if (! black[y][x]) {
        continue;
      }
      if (y == h-1) {
        down[y][x] = 1;
      } else {
        down[y][x] = down[y+1][x] + 1;
      }
    }
  }
  int ans = 0;
  auto solve = [&](int x, int y) {
    set<int> line;
    for (; x < w && y < h; x++, y++) {
      if (up[y][x] == 0) {
        line.clear();
        continue;
      }
      line.insert(x);
      int u = up[y][x];
      auto it = line.lower_bound(x - u + 1);
      while (it != line.end()) {
        int xl = *it;
        int size = x - xl + 1;
        int yl = y - size + 1;
        if (down[yl][xl] >= size) {
          ans = max(ans, size);
          break;
        } else {
          it = line.erase(it);
        }
      }
    }
  };
  REP (i,0,w) {
    solve(i, 0);
  }
  REP(i,1,h) {
    solve(0, i);
  }
  cout << ans << endl;

  return 0;
}

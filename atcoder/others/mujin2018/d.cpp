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

VI rev(int x) {
  if(x % 10 == 0) {
    return {};
  }
  int tr;
  if(x < 10) {
    tr = x;
    return {x, x * 10, x * 100};
  } else if(x < 100) {
    tr = (x % 10) * 10 + x / 10;
    return {tr, tr * 10};
  } else {
    tr = x / 100 + 10 * ((x / 10) % 10) + 100 * (x % 10);
    return {tr};
  }
}

void dfs(vector<vector<bool>> &visited, stack<PI> &st) {
  while(!st.empty()) {
    PI xy = st.top();
    st.pop();
    int x = xy.first;
    int y = xy.second;
    if(visited[x][y]) continue;
    visited[x][y] = true;
    if(x + y >= 1000) continue;
    VI rs = rev(x + y);
    FOR(r,rs) {
      if(r < y) {
        st.push({r, y});
      }
      if(x >= r) {
        st.push({x, r});
      }
    }
    VI rys = rev(y);
    FOR(ry,rys) {
      if(x + y >= ry) {
        st.push({x + y, ry});
      }
    }
    VI rxs = rev(x);
    FOR(rx,rxs) {
      if(rx < x + y) {
        st.push({rx, x + y});
      }
    }
  }
}

int main() {
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  vector<vector<bool>> visited(1000, vector<bool>(1000, false));
  stack<PI> st;
  REP(i,0,1000) {
    st.push({0, i});
    st.push({i, 0});
  }
  dfs(visited, st);
  int ans = 0;
  REP(i,1,n+1) {
    REP(j,1,m+1) {
      if(!visited[i][j]) {
        ans++;
      }
    }
  }
  cout << ans << endl;

  return 0;
}

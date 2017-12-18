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

bool reachable(const VI &set, int x) {
  if(set.size() == 0) {
    return x == 0;
  }
  int sum = accumulate(set.begin(), set.end(), 0);
  if(sum < x || sum + x < 0) {
    return false;
  }
  vector<vector<bool>> dp(set.size()+1, vector<bool>(sum*2+1, false));
  dp[0][sum] = true;
  REP(i,0,set.size()) {
    REP(j,0,sum*2+1) {
      int x1 = j - set[i];
      int x2 = j + set[i];
      dp[i+1][j] = (x1 >= 0 && dp[i][x1]) || (x2 <= sum * 2 && dp[i][x2]);
    }
  }
  return dp[set.size()][sum+x];
}

int main() {
  ios::sync_with_stdio(false);

  string s;
  cin >> s;
  int l = s.size();
  int x, y;
  cin >> x >> y;
  vector<VI> ops(2);
  int sx = 0;
  while(sx < l && s[sx] == 'F') sx++;
  x -= sx;
  int dir = 0;
  int d = 0;
  REP(i,sx,l) {
    if(s[i] == 'T') {
      if(d > 0) {
        ops[dir].push_back(d);
        d = 0;
      }
      dir = 1 - dir;
    } else {
      d++;
    }
  }
  if(d > 0) {
    ops[dir].push_back(d);
    d = 0;
  }
  cout << (reachable(ops[0], x) && reachable(ops[1], y) ? "Yes" : "No") << endl;
  
  return 0;
}

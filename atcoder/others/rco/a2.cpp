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
#include <unordered_map>
#include <queue>
#include <stack>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

const int DIRI[] = {0, 1, 0, -1};
const int DIRJ[] = {1, 0, -1, 0};

typedef pair<int,pair<int,int>> point;

// struct point {
//   int n;
//   int i;
//   int j;
//   point(int n, int i, int j) : n(n), i(i), j(j) {
//   }
//   int operator<(const point &p) {
//     return n > p.n;
//   }
// };

int main() {
  ios::sync_with_stdio(false);
  int H, W, K;
  cin >> H >> W >> K;
  vector<string> field(H);
  vector<point> ps;
  FOR(s,field) {
    cin >> s;
  }
  REP(i,0,H) {
    REP(j,0,W) {
      ps.push_back({field[i][j]-'0', {i, j}});
    }
  }
  sort(ps.rbegin(), ps.rend());
  vector<vector<PI>> ans;
  vector<vector<bool>> used(H, vector<bool>(W, false));
  FOR(p, ps) {
    if(used[p.second.first][p.second.second]) continue;
    vector<PI> v(1, p.second);
    priority_queue<point> q;
    REP(c,1,K) {
      used[p.second.first][p.second.second] = true;
      REP(d,0,4) {
        int ni = p.second.first + DIRI[d];
        int nj = p.second.second + DIRJ[d];
        if(ni < 0 || ni >= H || nj < 0 || nj >= W ||
           used[ni][nj] ||
           field[ni][nj] == '0') {
          continue;
        }
        q.push({field[ni][nj]-'0', {ni, nj}});
      }
      if(q.empty()) {
        v.clear();
        break;
      }
      point np = q.top();
      q.pop();
      v.push_back(np.second);
      p = np;
    }
    if((int)v.size() == K) {
      ans.push_back(v);
    }
  }
  cout << ans.size() << endl;
  FOR(v,ans) {
    FOR(p,v) {
      cout << p.first + 1 << " " << p.second + 1 << endl;
    }
  }
  return 0;
}

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

const int DIR[][2] = {{0, 1}, {0, -1}, {1, 0}, {-1, 0}};
const int INF = 1e9;

int main() {
  ios::sync_with_stdio(false);
  int h, w;
  cin >> h >> w;
  vector<string> fld(h);
  int white = 0;
  FOR(f,fld) {
    cin >> f;
    white += count(f.begin(), f.end(), '.');
  }
  queue<PI> que;
  que.push({0, 0});
  vector<VI> dist(h, VI(w, INF));
  dist[0][0] = 0;
  while(!que.empty()) {
    PI p = que.front();
    int i = p.first;
    int j = p.second;
    que.pop();
    REP(d,0,4) {
      int di = i + DIR[d][0];
      int dj = j + DIR[d][1];
      if(!(di >= 0 && di < h && dj >= 0 && dj < w)) {
        continue;
      }
      if(fld[di][dj] == '#' || dist[i][j] + 1 >= dist[di][dj]) {
        continue;
      }
      dist[di][dj] = dist[i][j] + 1;
      que.push({di, dj});
    }
  }
  if(dist[h-1][w-1] == INF) {
    cout << -1 << endl;
  } else {
    cout << white - dist[h-1][w-1] - 1 << endl;
  }
  
  return 0;
}

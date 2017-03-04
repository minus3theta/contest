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

struct uf {
  VI parent;
  VI count;
  uf(int n) : parent(n), count(n, 1) {
    REP(i,0,n) {
      parent[i] = i;
    }
  }
  int find(int x) {
    if(parent[x] == x) return x;
    return parent[x] = find(parent[x]);
  }
  int get_count(int x) {
    return count[find(x)];
  }
  void unite(int x, int y) {
    int rx = find(x);
    int ry = find(y);
    if(rx == ry) return;
    int cy = get_count(y);
    parent[y] = rx;
    count[rx] += cy;
  }
};

struct edge {
  int w;
  PI x;
  PI y;
  edge(int w, PI x, PI y) : w(w), x(x), y(y) {}
  bool operator<(const edge &e) {
    return w < e.w;
  }
};

int p2i(PI x, int W) {
  return x.first * W + x.second;
}

int main() {
  ios::sync_with_stdio(false);
  int H, W, K;
  cin >> H >> W >> K;
  vector<string> field(H);
  FOR(s,field) {
    cin >> s;
  }
  vector<edge> es;
  REP(i,0,H) {
    REP(j,0,W-1) {
      if(field[i][j] == '0' || field[i][j+1] == '0') continue;
      es.emplace_back((field[i][j]-'0') * (field[i][j+1]-'0'),
                      make_pair(i,j), make_pair(i,j+1));
    }
  }
  REP(i,0,H-1) {
    REP(j,0,W) {
      if(field[i][j] == '0' || field[i+1][j] == '0') continue;
      es.emplace_back((field[i][j]-'0') * (field[i+1][j]-'0'),
                      make_pair(i,j), make_pair(i+1,j));
    }
  }
  sort(es.rbegin(), es.rend());
  uf u(H * W);
  vector<vector<PI>> ans;
  FOR(e,es) {
    int x = p2i(e.x, W);
    int y = p2i(e.y, W);
    if(u.find(x) == u.find(y)) continue;
    if(u.get_count(x) + u.get_count(y) <= K) {
      // int cx = u.get_count(p2i(e.x, W));
      // int cy = u.get_count(p2i(e.y, W));
      u.unite(x, y);
      // if(cx + cy != u.get_count(p2i(e.x, W))) {
        // cout << e.x.first << " , " << e.x.second << endl;
        // cout << cx << " + " << cy << " = " << u.get_count(p2i(e.x, W)) << endl;
      // }
    }
  }
  map<int, vector<PI>> mp;
  REP(i,0,H) {
    REP(j,0,W) {
      int x = p2i({i,j}, W);
      if(u.get_count(x) == K) {
        mp[u.find(x)].push_back({i,j});
      }
      // printf("%04d ", u.get_count(x) == K ? u.find(x) : -1);
    }
    // printf("\n");
  }
  int count = 0;
  for(auto i=mp.begin(); i!=mp.end(); i++) {
    if((int)i->second.size() == K) {
      count++;
    }
  }
  cout << count << endl;
  for(auto i=mp.begin(); i!=mp.end(); i++) {
    if((int)i->second.size() != K) continue;
    FOR(p, i->second) {
      cout << p.first+1 << " " << p.second+1 << endl;
    }
  }
  return 0;
}

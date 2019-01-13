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
struct UnionFind {
  vector<int> par;
  vector<int> rank;
  UnionFind(int n): par(n), rank(n) {
    for(int i=0; i<n; i++) {
      par[i] = i;
      rank[i] = 0;
    }
  }
  int find(int x) {
    if(par[x] == x) {
      return x;
    }
    return par[x] = find(par[x]);
  }
  void unite(int x, int y) {
    x = find(x);
    y = find(y);
    if(rank[x] < rank[y]) {
      par[x] = y;
    } else {
      par[y] = x;
      if(rank[x] == rank[y]) {
        rank[x]++;
      }
    }
  }
  bool same(int x, int y) {
    return find(x) == find(y);
  }
};

int main() {
  ios::sync_with_stdio(false);
  int h, w;
  cin >> h >> w;
  vector<string> ss(h);
  FOR(s,ss) {
    cin >> s;
  }
  UnionFind uf(h * w);
  REP(i,0,h-1) {
    REP(j,0,w) {
      if (ss[i][j] != ss[i+1][j]) {
        uf.unite(i * w + j, (i + 1) * w + j);
      }
    }
  }
  REP(i,0,h) {
    REP(j,0,w-1) {
      if (ss[i][j] != ss[i][j+1]) {
        uf.unite(i * w + j, i * w + j + 1);
      }
    }
  }
  VL black(h * w, 0);
  VL white(h * w, 0);
  REP(i,0,h) {
    REP(j,0,w) {
      if (ss[i][j] == '#') {
        black[uf.find(i * w + j)]++;
      } else {
        white[uf.find(i * w + j)]++;
      }
    }
  }
  ll ans = 0;
  REP(i,0,h) {
    REP(j,0,w) {
      int id = i * w + j;
      if (id == uf.find(id)) {
        ans += black[id] * white[id];
      }
    }
  }
  cout << ans << endl;

  return 0;
}

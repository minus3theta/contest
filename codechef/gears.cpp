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

ll gcd(ll x, ll y) {
  if (y == 0) {
    return x;
  }
  return gcd(y, x % y);
}

void put_speed(ll v, ll a, ll b) {
  ll g = gcd(v * a, b);
  cout << v * a / g << "/" << b / g << endl;
}

int main() {
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  VL a(n);
  FOR(x, a) {
    cin >> x;
  }
  UnionFind uf(n * 2);
  REP(i,0,m) {
    int t;
    cin >> t;
    switch (t) {
    case 1:
      {
        int x;
        ll c;
        cin >> x >> c;
        a[x - 1] = c;
      }
      break;
    case 2:
      {
        int x, y;
        cin >> x >> y;
        x--;
        y--;
        uf.unite(x * 2, y * 2 + 1);
        uf.unite(x * 2 + 1, y * 2);
      }
      break;
    case 3:
      {
        int x, y;
        ll v;
        cin >> x >> y >> v;
        x--;
        y--;
        bool same = uf.same(x * 2, y * 2);
        bool opposite = uf.same(x * 2, y * 2 + 1);
        if (same && !opposite) {
          put_speed(v, a[x], a[y]);
        } else if (!same && opposite) {
          cout << "-";
          put_speed(v, a[x], a[y]);
        } else {
          cout << 0 << endl;
        }
      }
      break;
    default:
      break;
    }
  }

  return 0;
}

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
  vector<int> size;
  UnionFind(int n): par(n), size(n) {
    for(int i=0; i<n; i++) {
      par[i] = i;
      size[i] = 1;
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
    if (x == y) return;
    if(size[x] < size[y]) {
      par[x] = y;
      size[y] += size[x];
    } else {
      par[y] = x;
      size[x] += size[y];
    }
  }
  bool same(int x, int y) {
    return find(x) == find(y);
  }
  int getSize(int x) {
    return size[find(x)];
  }
};

int main() {
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  vector<PI> br(m);
  FOR(p,br) {
    cin >> p.first >> p.second;
    p.first--;
    p.second--;
  }
  reverse(br.begin(), br.end());
  VL ans;
  UnionFind uf(n);
  ll x = ((ll)n) * (n - 1) / 2;
  FOR(ab,br) {
    ans.push_back(x);
    int a = ab.first;
    int b = ab.second;
    if (!uf.same(a, b)) {
      ll sa = uf.getSize(a);
      ll sb = uf.getSize(b);
      x -= sa * sb;
      uf.unite(a, b);
    }
  }
  reverse(ans.begin(), ans.end());
  FOR(a,ans) {
    cout << a << endl;
  }

  return 0;
}

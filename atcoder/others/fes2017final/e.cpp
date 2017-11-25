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

int main() {
  ios::sync_with_stdio(false);
  string s;
  cin >> s;
  int l = s.size();
  VI xs(l+2);
  xs[0] = xs[l+1] = 0;
  transform(s.begin(), s.end(), xs.begin()+1, [](char c){return c-'a';});
  transform(xs.begin(), xs.end()-1, xs.begin()+1, xs.begin(), [](int x, int y){return (y+26-x)%26;});
  xs.pop_back();
  UnionFind u(l+1);
  int N;
  cin >> N;
  REP(i,0,N) {
    int l, r;
    cin >> l >> r;
    u.unite(l-1, r);
  }
  for(int i=0, j=l; i < j; i++, j--) {
    u.unite(i, j);
  }
  VI sum(l+1, 0);
  REP(i,0,l+1) {
    sum[u.find(i)] += xs[i];
    sum[u.find(i)] %= 26;
  }
  bool ans = all_of(sum.begin(), sum.end(), [](int x){return x == 0;});
  cout << (ans ? "YES" : "NO") << endl;

  return 0;
}

#include <cstdio>
#include <iostream>
#include <string>
#include <algorithm>
#include <numeric>
#include <functional>
#include <list>
#include <map>
#include <queue>
#include <stack>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long ll;

class uf {
public:
  vector<int> par;
  uf(int n) : par(n) {
    REP(i,0,n) {
      par[i] = i;
    }
  }
  int find(int x) {
    if(par[x] == x) return x;
    return par[x] = find(par[x]);
  }
  void unite(int x, int y) {
    par[find(x)] = find(y);
  }
  bool same(int x, int y) {
    return find(x) == find(y);
  }
};

struct edge {
  int s, t, c;
};

bool operator<(const edge &e1, const edge &e2) {
  return e1.c < e2.c;
}

int main() {
  int n;
  while(cin >> n) {
    uf u(n);
    vector<edge> es;
    REP(i,0,n) {
      REP(j,0,n) {
        int c;
        cin >> c;
        if(i<j) {
          es.push_back((edge){i,j,c});
        }
      }
    }
    sort(es.begin(), es.end());
    int cost = 0;
    REP(i,0,es.size()) {
      // cout << "p";
      if(u.same(es[i].s, es[i].t)) {
        continue;
      }
      // cout << "o";
      cost += es[i].c;
      u.unite(es[i].s, es[i].t);
      // cout << "he" << endl;
    }
    cout << cost << endl;
  }
  return 0;
}

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
  uf(int n): par(n) {
    REP(i,0,n) {
      par[i] = i;
    }
  }
  int find(int x) {
    if(par[x] == x) {
      return x;
    }
    return par[x] = find(par[x]);
  }
  void unite(int x, int y) {
    par[find(x)] = find(y);
  }
  bool same(int x, int y) {
    return find(x) == find(y);
  }
};

struct c {
  int x, y;
  vector<int> nb;
  bool alive;
};

int main() {
  int n, d;
  cin >> n >> d;
  vector<c> cs(n+1);
  REP(i,1,n+1) {
    cin >> cs[i].x >> cs[i].y;
    cs[i].alive = false;
  }
  REP(i,1,n+1) {
    REP(j,i+1,n+1) {
      if((cs[i].x-cs[j].x)*(cs[i].x-cs[j].x) +
         (cs[i].y-cs[j].y)*(cs[i].y-cs[j].y) <= d*d) {
        cs[i].nb.push_back(j);
        cs[j].nb.push_back(i);
      }
    }
  }
  uf u(n+1);
  string s;
  while(cin >> s) {
    if(s == "O") {
      int p;
      cin >> p;
      cs[p].alive = true;
      int l = cs[p].nb.size();
      REP(i,0,l) {
        int q = cs[p].nb[i];
        if(cs[q].alive) {
          u.unite(p, q);
        }
      }
    } else {
      int p, q;
      cin >> p >> q;
      cout << (u.same(p, q) ? "SUCCESS" : "FAIL") << endl;
    }
  }

  return 0;
}

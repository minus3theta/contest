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
  uf(int n): par(n){
    REP(i,0,n) {
      par[i] = i;
    }
  }
  void unite(int, int);
  int find(int);
  bool same(int, int);
};

void uf::unite(int x, int y) {
  par[find(x)] = y;
}
int uf::find(int x) {
  if(par[x] == x) {
    return x;
  }
  return par[x] = find(par[x]);
}
bool uf::same(int x, int y) {
  return find(x) == find(y);
}

struct cp {
  int x, y;
  vector<int> nb;
  bool alive;
};

int main() {
  int N, d;
  cin >> N >> d;
  vector<cp> cps(N+1);
  uf u(N+1);
  REP(i,1,N+1) {
    cin >> cps[i].x >> cps[i].y;
    cps[i].alive = false;
  }
  REP(i,1,N+1) {
    REP(j,i+1,N+1) {
      if((cps[i].x-cps[j].x)*(cps[i].x-cps[j].y) +
         (cps[i].y-cps[j].y)*(cps[i].y-cps[j].y) <= d*d) {
        cps[i].nb.push_back(j);
        cps[j].nb.push_back(i);
      }
    }
  }
  string op;
  while(cin >> op) {
    if(op == "O") {
      int r;
      cin >> r;
      cps[r].alive = true;
      REP(i,0,cps[r].nb.size()) {
        int x = cps[r].nb[i];
        if(cps[x].alive) {
          u.unite(r, x);
        }
      }
    } else if(op == "S") {
      int p, q;
      cin >> p >> q;
      cout << (u.same(p, q) ? "SUCCESS" : "FAIL") << endl;
    }
  }
  return 0;
}

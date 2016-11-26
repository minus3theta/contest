#include <cstdio>
#include <climits>
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

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

struct uf {
  VI par;
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

int main() {
  int N, M;
  cin >> N >> M;
  uf u(M+1);
  vector<VI> peo(N);
  int mink = 1e6;
  int minkp = -1;
  REP(i,0,N) {
    int K;
    cin >> K;
    if(K < mink) {
      mink = K;
      minkp = i;
    }
    int l1 = 0;
    REP(j,0,K) {
      int l;
      cin >> l;
      peo[i].push_back(l);
      if(l1) {
        u.unite(l1,l);
      } else {
        l1 = l;
      }
    }
  }
  bool pos = true;
  REP(i,0,N) {
    if(i == minkp) continue;
    bool posi = false;
    for(const auto &l1 : peo[minkp]) {
      for(const auto &l2 : peo[i]) {
        if(u.same(l1, l2)) {
          posi = true;
        }
      }
    }
    if(!posi) {
      pos = false;
      break;
    }
  }
  cout << (pos ? "YES\n" : "NO\n");

  return 0;
}

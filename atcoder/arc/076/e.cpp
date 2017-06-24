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

bool is_edge(ll R, ll C, ll x, ll y) {
  return x == 0 || x == R || y == 0 || y == C;
}

struct num {
  ll x1, y1, x2, y2;
};

struct point {
  ll x, y;
  int id;
};

int place(ll R, ll C, const point &p) {
  if(p.y == 0) return 0;
  if(p.x == R) return 1;
  if(p.y == C) return 2;
  return 3;
}

int main() {
  ios::sync_with_stdio(false);
  ll R, C, N;
  cin >> R >> C >> N;
  vector<num> ns(N);
  FOR(n,ns) {
    cin >> n.x1 >> n.y1 >> n.x2 >> n.y2;
  }
  vector<point> ps;
  REP(i,0,N) {
    num n = ns[i];
    if(is_edge(R, C, n.x1, n.y1) && is_edge(R, C, n.x2, n.y2)) {
      ps.push_back({n.x1, n.y1, i});
      ps.push_back({n.x2, n.y2, i});
    }
  }
  sort(ps.begin(), ps.end(), [=](point p, point q) {
      int pp = place(R, C, p);
      int pq = place(R, C, q);
      if(pp != pq) {
        return pp < pq;
      }
      switch(pp) {
      case 0:
        return p.x < q.x;
        break;
      case 1:
        return p.y < q.y;
        break;
      case 2:
        return p.x > q.x;
        break;
      default:
        return p.y > q.y;
      }
    });
  stack<int> st;
  FOR(p,ps) {
    if(st.empty() || st.top() != p.id) {
      st.push(p.id);
    } else {
      st.pop();
    }
  }
  cout << (st.empty() ? "YES" : "NO") << endl;
  return 0;
}

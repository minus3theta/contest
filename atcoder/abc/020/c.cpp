#include <cstdio>
#include <climits>
#include <cassert>
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

const ll INF = 1e18;

ll time(const vector<string> &f, const PI &sij, const PI &gij, ll x) {
  vector<VL> cost(f.size(), VL(f[0].size(), INF));
  priority_queue<pair<ll,PI>, vector<pair<ll,PI>>, greater<pair<ll,PI>>> q;
  q.push({0, sij});
  vector<PI> vs = {{1,0}, {-1,0}, {0,1}, {0,-1}};
  while(!q.empty()) {
    auto a = q.top();
    q.pop();
    int ai = a.second.first;
    int aj = a.second.second;
    if(a.first >= cost[ai][aj]) {
      continue;
    }
    cost[ai][aj] = a.first;
    if(a.second == gij) {
      break;
    }
    FOR(v,vs) {
      int bi = ai + v.first;
      int bj = aj + v.second;
      if(f[bi][bj] == '$') {
        continue;
      }
      int dt = f[bi][bj] == '#' ? x : 1;
      q.push({a.first + dt, {bi, bj}});
    }
  }
  return cost[gij.first][gij.second];
}

int main() {
  int H, W;
  ll T;
  cin >> H >> W >> T;
  vector<string> f(H+2);
  f[0] = string(W+2, '$');
  f[H+1] = string(W+2, '$');
  PI sij = {0, 0};
  PI gij = {0, 0};
  REP(i,1,H+1) {
    string s;
    cin >> s;
    f[i] = '$' + s + '$';
    unsigned long long sp = f[i].find_first_of('S');
    if(sp != string::npos) {
      sij = {i, sp};
    }
    unsigned long long gp = f[i].find_first_of('G');
    if(gp != string::npos) {
      gij = {i, gp};
    }
  }
  ll l = 0;
  ll r = T;
  while(l+1 != r) {
    ll m = (l + r) / 2;
    if(time(f, sij, gij, m) <= T) {
      l = m;
    } else {
      r = m;
    }
  }
  cout << l << endl;
  return 0;
}

#include <cstdio>
#include <climits>
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
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

const int INF = INT_MAX / 3;
const int MAXN = 10000;

VI itov(int x) {
  VI u(4);
  REP(i,0,4) {
    u[i] = x % 10;
    x /= 10;
  }
  return u;
}

int vtoi(const VI &u) {
  int x = 0;
  REP(i,0,u.size()) {
    x *= 10;
    x += u[u.size()-i-1];
  }
  return x;
}

int main() {
  VI v(MAXN, 1);
  v[0] = v[1] = 0;
  for(int i=2; i*i<MAXN; i++) {
    REP(j,i+1,MAXN) {
      if(!v[j]) continue;
      if(j % i == 0) {
        v[j] = 0;
      }
    }
  }
  REP(i,0,1000) {
    v[i] = 0;
  }
  vector<VI> e(MAXN);
  REP(i,0,MAXN) {
    if(!v[i]) continue;
    VI a = itov(i);
    REP(j,0,a.size()) {
      REP(k,0,10) {
        VI b = a;
        b[j] = k;
        int x = vtoi(b);
        if(v[x]) {
          e[i].push_back(x);
          e[x].push_back(i);
        }
      }
    }
  }
  int n;
  cin >> n;
  REP(i,0,n) {
    int s, t;
    cin >> s >> t;
    VI cost(MAXN, INF);
    queue<int> q;
    q.push(s);
    cost[s] = 0;

    // VI prev(MAXN,-1);
    while(!q.empty()) {
      int x = q.front();
      q.pop();
      if(x == t) break;
      REP(i,0,e[x].size()) {
        int y = e[x][i];
        if(cost[x] + 1 < cost[y]) {
          q.push(y);
          // prev[y] = x;
          cost[y] = cost[x] + 1;
        }
      }
    }
    if(cost[t] == INF) {
      cout << "Impossible\n";
    } else {
      cout << cost[t] << endl;
      // while(prev[t] != -1) {
      //   cout << t << endl;
      //   t = prev[t];
      // }
    }
  }
  return 0;
}

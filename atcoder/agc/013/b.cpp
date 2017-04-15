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

int main() {
  ios::sync_with_stdio(false);
  int N, M;
  cin >> N >> M;
  vector<VI> es(N+1);
  vector<bool> visit(N+1, false);
  REP(i,0,M) {
    int a, b;
    cin >> a >> b;
    es[a].push_back(b);
    es[b].push_back(a);
  }
  stack<int> ans1;
  queue<int> ans2;
  int x = 1;
  visit[x] = true;
  while(true) {
    ans1.push(x);
    bool dead = true;
    FOR(e,es[x]) {
      if(visit[e]) continue;
      visit[e] = true;
      x = e;
      dead = false;
      break;
    }
    if(dead) break;
  }
  x = 1;
  while(true) {
    ans2.push(x);
    bool dead = true;
    FOR(e,es[x]) {
      if(visit[e]) continue;
      visit[e] = true;
      x = e;
      dead = false;
      break;
    }
    if(dead) break;
  }
  ans2.pop();
  cout << ans1.size() + ans2.size() << endl;
  while(!ans1.empty()) {
    int a = ans1.top();
    ans1.pop();
    cout << a << " ";
  }
  while(!ans2.empty()) {
    int a = ans2.front();
    ans2.pop();
    cout << a << " ";
  }
  cout << endl;
  return 0;
}

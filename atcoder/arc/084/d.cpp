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
#include <deque>

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
  ll K;
  cin >> K;
  VL dist(K, INT_MAX);
  dist[1] = 1;
  deque<int> q;
  q.push_front(1);
  while(!q.empty()) {
    int x = q.front();
    q.pop_front();
    int b = (x * 10) % K;
    if(dist[x] < dist[b]) {
      dist[b] = dist[x];
      q.push_front(b);
    }
    int a = (x + 1) % K;
    if(dist[x] + 1 < dist[a]) {
      dist[a] = dist[x] + 1;
      q.push_back(a);
    }
  }
  cout << dist[0] << endl;
  
  return 0;
}

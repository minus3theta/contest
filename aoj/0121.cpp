#include <cstdio>
#include <iostream>
#include <string>
#include <algorithm>
#include <list>
#include <map>
#include <queue>
#include <stack>
#include <set>
#include <vector>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)

using namespace std;
typedef long long ll;

int dirs[][2] = {{-1, 0}, {1,0}, {0, -1}, {0, 1}};

int main() {
  map<vector<int>, ll> m;
  queue<pair<vector<int>, ll>> q;
  q.push({{0,1,2,3,4,5,6,7},0LL});
  m.insert({{0,1,2,3,4,5,6,7},0LL});
  while(!q.empty()) {
    pair<vector<int>, ll> t;
    t = q.front();
    q.pop();
    int pos0 = 0;
    REP(i,0,8) {
      if(t.first[i] == 0) {
        pos0 = i;
        break;
      }
    }
    int x = pos0 % 4;
    int y = pos0 / 4;
    REP(i,0,4) {
      int xx = x + dirs[i][0];
      int yy = y + dirs[i][1];
      if(xx < 0 || xx >= 4 || yy < 0 || yy >= 2) {
        continue;
      }
      vector<int> v = t.first;
      v[pos0] = t.first[xx + yy * 4];
      v[xx + yy * 4] = 0;
      if(m.find(v) == m.end()) {
        q.push({v, t.second + 1LL});
        m.insert({v, t.second + 1LL});
      }
    }
  }
  while(cin) {
    vector<int> init;
    REP(i,0,8) {
      int x;
      cin >> x;
      init.push_back(x);
    }
    if(cin.eof()) break;
    cout << m[init] << endl;
  }
  return 0;
}

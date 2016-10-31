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

bool cmp(const pair<int,int> &a, const pair<int,int> &b) {
  return a.second > b.second;
}

int main() {
  int c, l;
  cin >> c >> l;
  vector<pair<int,int> > cow(c);
  REP(i,0,c) {
    cin >> cow[i].first >> cow[i].second;
  }
  vector<pair<int,int> > lot(l);
  REP(i,0,l) {
    cin >> lot[i].first >> lot[i].second;
  }
  sort(cow.begin(), cow.end());
  sort(lot.begin(), lot.end());
  priority_queue<pair<int,int>, vector<pair<int,int> >,
                 bool (*)(const pair<int,int> &, const pair<int,int> &) > q(cmp);
  int ans = 0;
  int p = 0;
  REP(i,0,l) {
    while(p < c && cow[p].first <= lot[i].first) {
      q.push(cow[p++]);
    }
    while(!q.empty() && lot[i].second > 0) {
      pair<int,int> cw = q.top();
      q.pop();
      if(cw.second >= lot[i].first) {
        ans++;
        lot[i].second--;
      }
    }
  }
  cout << ans << endl;
  return 0;
}

#include <cstdio>
#include <iostream>
#include <cmath>
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

const double EPS = 1e-6;

int main() {
  ll cs = 1;
  while(true) {
    ll n, d;
    vector<pair<double,double> > v;
    cin >> n >> d;
    if(!n && !d) break;
    bool possible = true;
    REP(i,0,n) {
      ll x, y;
      cin >> x >> y;
      if(y > d) {
        possible = false;
      }
      double dx = sqrt(d * d - y * y);
      v.push_back(make_pair(x + dx, x - dx));
    }
    printf("Case %lld: ", cs++);
    sort(v.begin(), v.end());
    if(!possible) {
      printf("-1\n");
      continue;
    }
    ll c = 0;
    double rx = -1.0/0.0;
    REP(i,0,v.size()) {
      if(v[i].second < rx + EPS) {
        continue;
      } else {
        c++;
        rx = v[i].first;
      }
    }
    cout << c << endl;
  }
  return 0;
}

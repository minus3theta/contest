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

int main() {
  ll n, t;
  scanf("%lld %lld", &n, &t);
  vector<pair<ll,ll> > v(n);
  REP(i,0,n) {
    scanf("%lld %lld", &v[i].first, &v[i].second);
  }
  sort(v.begin(), v.end());
  ll cnt = 0;
  ll start = 1;
  ll x = 0;
  while(start <= t) {
    ll max = -1;
    while(x < n && v[x].first <= start) {
      max = v[x].second > max ? v[x].second : max;
      x++;
    }
    if(max < start) {
      cnt = -1;
      break;
    }
    start = max+1;
    cnt++;
  }
  printf("%lld\n", cnt);
  return 0;
}

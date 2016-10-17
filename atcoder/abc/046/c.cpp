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

ll divceil(ll x, ll y) {
  return (x + y - 1) / y;
}

int main() {
  ll n;
  cin >> n;
  vector<pair<ll,ll>> v(n);
  REP(i,0,n) {
    cin >> v[i].first >> v[i].second;
  }
  ll t = v[0].first;
  ll a = v[0].second;
  REP(i,1,n) {
    ll k = max(divceil(t, v[i].first), divceil(a, v[i].second));
    t = v[i].first * k;
    a = v[i].second * k;
  }
  cout << t + a << endl;
  return 0;
}

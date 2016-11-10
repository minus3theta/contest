#include <cstdio>
#include <climits>
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

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

ll gcd(ll a, ll b) {
  if(!b) return a;
  return gcd(b, a % b);
}

int main() {
  ll a, b;
  while(cin >> a >> b) {
    ll g = gcd(a, b);
    cout << g << " " << a * b / g << endl;
  }
  return 0;
}

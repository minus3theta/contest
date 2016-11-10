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

bool isprime(ll x) {
  if(x < 2) return false;
  for(int i=2; i*i <= x; i++) {
    if(x % i == 0) {
      return false;
    }
  }
  return true;
}

ll pow(ll p, ll a, ll mod) {
  if(p == 0) {
    return 1;
  }
  ll x = pow(p/2, a, mod);
  x = (x * x) % mod;
  if(p % 2) {
    return (x * a) % mod;
  }
  return x % mod;
}

int main() {
  ll p, a;
  while(true) {
    cin >> p >> a;
    if(!p && !a) break;
    if(pow(p, a, p) == a % p && !isprime(p)) {
      cout << "yes\n";
    } else {
      cout << "no\n";
    }
  }
  return 0;
}

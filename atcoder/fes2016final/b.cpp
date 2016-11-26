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

int main() {
  ll N;
  cin >> N;
  ll k = 0;
  ll total = 0;
  for(ll i=0; ; i++) {
    if((total = (i * (i+1)) / 2) >= N) {
      k = i;
      break;
    }
  }
  REP(i,1,k+1) {
    if(i != total - N) {
      cout << i << endl;
    }
  }
  return 0;
}

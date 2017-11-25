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

int solve(const VI &pop) {
  if(pop[0] != 0 || pop[12] >= 2) {
    return 0;
  }
  REP(i,1,12) {
    if(pop[i] >= 3) {
      return 0;
    }
  }
  int ans = -1;
  REP(i,0,1<<11) {
    VI place({0,24});
    if(pop[12]) {
      place.push_back(12);
    }
    REP(t,1,12) {
      if(pop[t] == 1) {
        if((i >> (t-1)) & 1) {
          place.push_back(t);
        } else {
          place.push_back(24-t);
        }
      } else if(pop[t] == 2) {
        place.push_back(t);
        place.push_back(24-t);
      }
    }
    sort(place.begin(), place.end());
    int m = 1000;
    REP(j,1,place.size()) {
      m = min(m, place[j] - place[j-1]);
    }
    ans = max(ans, m);
  }
  return ans;
}

int main() {
  ios::sync_with_stdio(false);
  int N;
  cin >> N;
  VI pop(13);
  REP(i,0,N) {
    int d;
    cin >> d;
    pop[d]++;
  }
  cout << solve(pop) << endl;
  
  return 0;
}

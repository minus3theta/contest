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
  int H, W;
  cin >> H >> W;
  int ii = 0, jj = 0;
  REP(i,0,H) {
    REP(j,0,W) {
      string s;
      cin >> s;
      if(s == "snuke") {
        ii = i;
        jj = j;
      }
    }
  }
  cout << (char)('A' + jj) << (ii+1) << endl;
      
  return 0;
}

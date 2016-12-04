#include <cstdio>
#include <climits>
#include <cassert>
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

int main() {
  ios::sync_with_stdio(false);
  string s;
  cin >> s;
  bool fst = false;
  while(true) {
    bool pos = false;
    REP(i,1,s.size()-1) {
      if(s[i-1] != s[i+1]) {
        s.erase(i,1);
        pos = true;
        break;
      }
    }
    if(pos) {
      fst = !fst;
    } else {
      break;
    }
  }
  cout << (fst ? "First\n" : "Second\n");
  return 0;
}

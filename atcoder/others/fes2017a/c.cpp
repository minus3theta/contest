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

bool ee(int mod[]) {
  return mod[1] == 0 && mod[2] == 0 && mod[3] == 0;
}

bool eo(int mod[], int e) {
  return mod[1] == 0 && mod[3] == 0 && mod[2] <= e / 2;
}

bool oo(int mod[], int x, int y) {
  if(mod[1] == 0 && mod[3] == 1) {
      mod[2]++;
  } else if(!(mod[1] == 1 && mod[3] == 0)) {
    return false;
  }
  return mod[2] <= x / 2 + y / 2;
}

int main() {
  ios::sync_with_stdio(false);
  int H, W;
  cin >> H >> W;
  int count[26] = {};
  REP(i,0,H) {
    string s;
    cin >> s;
    FOR(c,s) {
      count[c-'a']++;
    }
  }
  int mod[4] = {};
  REP(i,0,26) {
    mod[count[i] % 4]++;
  }
  bool ans;
  if(H % 2 == 0 && W % 2 == 0) {
    ans = ee(mod);
  } else if(H % 2 == 1 && W % 2 == 1) {
    ans = oo(mod, H, W);
  } else if(H % 2 == 1) {
    ans = eo(mod, W);
  } else {
    ans = eo(mod, H);
  }
  cout << (ans ? "Yes" : "No") << endl;
  return 0;
}

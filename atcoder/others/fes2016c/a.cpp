#include <cstdio>
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

int main() {
  string s;
  cin >> s;
  bool c = false;
  bool f = false;
  REP(i,0,s.size()) {
    if(!c && s[i] == 'C') {
      c = true;
    } else if(c && s[i] == 'F') {
      f = true;
    }
  }
  cout << (f ? "Yes" : "No") << endl;
  return 0;
}

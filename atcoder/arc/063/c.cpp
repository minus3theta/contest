#include <cstdio>
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

int main() {
  string s;
  cin >> s;
  int x = 0;
  char c = s[0];
  REP(i,1,s.size()) {
    if(c != s[i]) {
      x++;
      c = s[i];
    }
  }
  cout << x << endl;
  return 0;
}

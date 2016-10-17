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

int main() {
  string s;
  cin >> s;
  ll c = 0;
  int n = s.size();
  REP(i,0,n) {
    if(s[i] == 'p') {
      c++;
    }
  }
  cout << n / 2 - c << endl;
  return 0;
}

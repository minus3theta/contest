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
  string a = "CODEFESTIVAL2016";
  string s;
  cin >> s;
  int c = 0;
  REP(i,0,16) {
    if(a[i] != s[i]) {
      c++;
    }
  }
  cout << c << endl;
  return 0;
}

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
  ll n, a, b;
  string s;
  cin >> n >> a >> b >> s;
  ll p = 0;
  ll pa = 1;
  REP(i,0,n) {
    if(s[i] == 'a') {
      if(p < a + b) {
        cout << "Yes\n";
        p++;
      } else {
        cout << "No\n";
      }
    } else if(s[i] == 'b') {
      if(p < a + b && pa <= b) {
        cout << "Yes\n";
        p++;
        pa++;
      } else {
        cout << "No\n";
      }
    } else {
      cout << "No\n";
    }
  }
  return 0;
}

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
  vector<int> v(101);
  REP(i,0,3) {
    int x;
    cin >> x;
    v[x]++;
  }
  int c = 0;
  REP(i,0,v.size()) {
    if(v[i]) c++;
  }
  cout << c << endl;
  return 0;
}

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
  int n, k;
  cin >> n >> k;
  ll x = k;
  REP(i,1,n) {
    x *= k - 1;
  }
  cout << x << endl;
  return 0;
}

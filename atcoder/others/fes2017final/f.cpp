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

int main() {
  ios::sync_with_stdio(false);
  int P = 37;
  int K = P + 1;
  int N = P * K + 1;
  cout << N << " " << K << endl;;
  REP(i,0,K) {
    cout << N << " ";
    REP(j,0,P) {
      cout << i * P + j + 1 << " ";
    }
    cout << endl;
  }
  REP(i,0,P) {
    REP(j,0,P) {
      cout << N - P + i << " ";
      REP(k,0,P) {
        cout << k * P + (i * k + j) % P + 1 << " ";
      }
      cout << endl;
    }
  }
      
  return 0;
}

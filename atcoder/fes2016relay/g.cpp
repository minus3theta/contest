#include <cstdio>
#include <climits>
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
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

int main() {
  int N, Q;
  cin >> N >> Q;
  vector<PI> qs(Q);
  for(auto &q: qs) {
    cin >> q.first >> q.second;
  }
  VI cup(N+2, 0);
  int pos = 1;
  cup[1] = 1;
  cup[2] = 1;
  for(auto &q: qs) {
    int a = q.first;
    int b = q.second;
    swap(cup[a], cup[b]);
    if(a == pos) {
      pos = b;
    } else if(b == pos) {
      pos = a;
    }
    if(pos-1 >= 1) {
      cup[pos-1] = 1;
    }
    if(pos+1 <= N) {
      cup[pos+1] = 1;
    }
  }
  int ans = 0;
  REP(i,1,N+1) {
    if(cup[i]) ans++;
  }
  cout << ans << endl;
  return 0;
}

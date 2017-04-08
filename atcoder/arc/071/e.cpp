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

struct query {
  int a, b, c, d;
};


int main() {
  ios::sync_with_stdio(false);
  string S, T;
  cin >> S >> T;
  int Q;
  cin >> Q;
  vector<query> qs(Q);
  FOR(q,qs) {
    cin >> q.a >> q.b >> q.c >> q.d;
  }
  VI a_in_S(S.size()+1, 0);
  VI a_in_T(T.size()+1, 0);
  REP(i,0,S.size()) {
    a_in_S[i+1] = a_in_S[i] + (S[i] == 'A' ? 1 : 2);
  }
  REP(i,0,T.size()) {
    a_in_T[i+1] = a_in_T[i] + (T[i] == 'A' ? 1 : 2);
  }
  FOR(q,qs) {
    int s = a_in_S[q.b] - a_in_S[q.a-1];
    int t = a_in_T[q.d] - a_in_T[q.c-1];
    bool ans = s % 3 == t % 3;
    if(s == 0 && t != 0) ans = false;
    cout << (ans ? "YES" : "NO") << endl;
  }
  return 0;
}

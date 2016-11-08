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

const ll INF = LONG_MAX / 3;

class tree {
public:
  vector<vector<int>> nb;
  vector<pair<ll,ll>> in;
  vector<int> val;
  int s;
  tree(int N) : nb(N+1), in(N+1,{-INF,INF}), val(N+1) {
    REP(i,0,N-1) {
      int a, b;
      cin >> a >> b;
      nb[a].push_back(b);
      nb[b].push_back(a);
    }
    int K;
    cin >> K;
    REP(i,0,K) {
      int a, b;
      cin >> a >> b;
      in[a] = {b,b};
      s = a;
    }
  }
  bool assign(int x, bool odd, int from) {
    for(int v: nb[x]) {
      if(v == from) continue;
      if(!assign(v, !odd, x)) {
        return false;
      }
      in[x].first = max(in[x].first, in[v].first-1);
      in[x].second = min(in[x].second, in[v].second+1);
    }
    if(in[x].first % 2 != odd) {
      in[x].first++;
    }
    if(in[x].second % 2 != odd) {
      in[x].second--;
    }
    return in[x].first <= in[x].second;
  }
  void write(int x, int p, int from) {
    if(in[x].first <= p-1 && p-1 <= in[x].second) {
      val[x] = p-1;
    } else {
      val[x] = p+1;
    }
    for(int v: nb[x]) {
      if(v == from) continue;
      write(v, val[x], x);
    }
  }
};

int main() {
  int N;
  cin >> N;
  tree t(N);
  if(t.assign(t.s, t.in[t.s].first % 2, -1)) {
    cout << "Yes\n";
    t.write(t.s, t.in[t.s].first - 1, -1);
    REP(i,1,N+1) {
      cout << t.val[i] << "\n";
    }
  } else {
    cout << "No\n";
  }
  return 0;
}

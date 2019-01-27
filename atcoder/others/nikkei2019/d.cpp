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
#include <iterator>
#include <regex>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

template <class T, template <class, class> class C, class charT = char>
void vdump(const C<T, allocator<T>> &v, const charT* delimiter = ", ",
           ostream &stream = cout) {
  copy(v.begin(), v.end(), ostream_iterator<T>(stream, delimiter));
  stream << endl;
}

void dfs(int x, const vector<VI> &es, VI &top, int &current) {
  FOR(v,es[x]) {
    if(top[v]) {
      continue;
    }
    dfs(v, es, top, current);
  }
  top[x] = current--;
}

int main() {
  ios::sync_with_stdio(false);
  int n, m;
  cin >> n >> m;
  vector<VI> es(n+1);
  vector<VI> rev(n+1);
  REP(i,0,n-1+m) {
    int a, b;
    cin >> a >> b;
    es[a].push_back(b);
    rev[b].push_back(a);
  }
  int current = n;
  VI top(n+1);
  int root = 0;
  REP(i,1,n+1) {
    if(rev[i].empty()) {
      root = i;
      break;
    }
  }
  dfs(root, es, top, current);
  REP(i,1,n+1) {
    int parent = 0;
    int maxt = -1;
    FOR(v,rev[i]) {
      if(top[v] > maxt) {
        parent = v;
        maxt = top[v];
      }
    }
    cout << parent << endl;
  }

  return 0;
}

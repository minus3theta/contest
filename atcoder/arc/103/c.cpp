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

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  VI vs(n);
  FOR(v,vs) {
    cin >> v;
  }
  vector<vector<PI>> pop(2, vector<PI>(1e5+1, {0, 0}));
  REP(i,0,2) {
    for(int j=0; j<=1e5; j++) {
      pop[i][j].second = j;
    }
  }
  REP(i,0,2) {
    for(int j=i; j<n; j += 2) {
      pop[i][vs[j]].first++;
    }
  }
  REP(i,0,2) {
    sort(pop[i].rbegin(), pop[i].rend());
  }
  int keep;
  if(pop[0][0].second != pop[1][0].second) {
    keep = pop[0][0].first + pop[1][0].first;
  } else {
    keep = max(pop[0][0].first + pop[1][1].first,
               pop[0][1].first + pop[1][0].first);
  }
  cout << n - keep << endl;

  return 0;
}

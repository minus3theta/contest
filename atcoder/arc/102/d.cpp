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

struct eg {
  int src, dst, w;
};

int construct(int l, vector<eg> &es) {
  if(l == 1) {
    return 1;
  }
  if(l == 2) {
    construct(1, es);
    es.push_back({1, 2, 0});
    es.push_back({1, 2, 1});
    return 2;
  }
  if(l % 3 == 0) {
    int sz = construct(l / 3, es);
    es.push_back({sz, sz + 1, 0});
    es.push_back({sz, sz + 1, l / 3});
    es.push_back({sz, sz + 1, (l / 3) * 2});
    return sz + 1;
  }
  if(l % 3 == 1) {
    int sz = construct(l - 1, es);
    es.push_back({1, sz, l - 1});
    return sz;
  }
  int sz = construct(l - 2, es);
  es.push_back({1, sz, l - 1});
  es.push_back({1, sz, l - 2});
  return sz;
}

int main() {
  ios::sync_with_stdio(false);
  int l;
  cin >> l;
  vector<eg> es;
  int n = construct(l, es);
  cout << n << " " << es.size() << endl;
  FOR(e,es) {
    cout << e.src << " " << e.dst << " " << e.w << endl;
  }

  return 0;
}

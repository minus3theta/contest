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
  int n, q;
  cin >> n >> q;
  vector<tuple<ll,ll,ll>> stop(n);
  FOR(a,stop) {
    ll s, t, x;
    cin >> s >> t >> x;
    a = make_tuple(s, t, x);
  }
  VL ds(q);
  FOR(d,ds) {
    cin >> d;
  }
  vector<tuple<ll, bool, ll>> interval;
  FOR(a,stop) {
    interval.emplace_back(get<0>(a) - get<2>(a), true, get<2>(a));
    interval.emplace_back(get<1>(a) - get<2>(a), false, get<2>(a));
  }
  sort(interval.begin(), interval.end());
  multiset<ll> current_belong;
  int person = 0;
  FOR(i,interval) {
    while (person < q && ds[person] < get<0>(i)) {
      if (current_belong.size() == 0) {
        cout << -1 << endl;
      } else {
        cout << *current_belong.begin() << endl;
      }
      person++;
    }
    if (get<1>(i)) {
      current_belong.insert(get<2>(i));
    } else {
      current_belong.erase(get<2>(i));
    }
  }
  while (person < q) {
    cout << -1 << endl;
    person++;
  }

  return 0;
}

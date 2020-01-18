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
  vector<set<int>> fol(n);
  REP(i,0,q) {
    int t;
    cin >> t;
    if (t == 1) {
      int a, b;
      cin >> a >> b;
      a--;
      b--;
      fol[a].insert(b);
    } else if (t == 2) {
      int a;
      cin >> a;
      a--;
      REP(b,0,n) {
        if (fol[b].find(a) != fol[b].end()) {
          fol[a].insert(b);
        }
      }
    } else {
      int a;
      cin >> a;
      a--;
      set<int> add;
      FOR(b,fol[a]) {
        FOR(x,fol[b]) {
          if (x != a) {
            add.insert(x);
          }
        }
      }
      FOR(x,add) {
        fol[a].insert(x);
      }
    }
  }
  REP(i,0,n) {
    REP(j,0,n) {
      if (fol[i].find(j) != fol[i].end()) {
        cout << 'Y';
      } else {
        cout << 'N';
      }
    }
    cout << endl;
  }

  return 0;
}

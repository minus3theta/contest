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
// 0-indexed
template <class T, class Op = T (*) (T, T)>
struct Segtree {
  int n;
  vector<T> dat;
  Op op;
  T unit;
  Segtree(int al, Op op = Op(), T unit = T()): op(op), unit(unit) {
    n = 1;
    while(n < al) {
      n *= 2;
    }
    dat = vector<T>(2 * n - 1, unit);
  }
  Segtree(const vector<T> &arr, Op op = Op(), T unit = T()): op(op), unit(unit) {
    int al = arr.size();
    n = 1;
    while(n < al) {
      n *= 2;
    }
    dat = vector<T>(2 * n - 1, unit);
    copy(arr.begin(), arr.end(), dat.begin() + n - 1);
    for(int i=n-2; i >= 0; i--) {
      dat[i] = op(dat[i * 2 + 1], dat[i * 2 + 2]);
    }
  }
  void update(int k, T x) {
    k += n - 1;
    dat[k] = x;
    while(k > 0) {
      k = (k - 1) / 2;
      dat[k] = op(dat[k * 2 + 1], dat[k * 2 + 2]);
    }
  }
  // accumlate [a, b)
  T accum(int a, int b, int k = 0, int l = 0, int r = -1) {
    if(r < 0) r = n;
    if(r <= a || b <= l) return unit;
    if(a <= l && r <= b) return dat[k];
    T vl = accum(a, b, k * 2 + 1, l, (l + r) / 2);
    T vr = accum(a, b, k * 2 + 2, (l + r) / 2, r);
    return op(vl, vr);
  }
};

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  string s;
  cin >> s;
  int q;
  cin >> q;
  auto cl = [](int x, int y){return x | y;};
  Segtree<int, decltype(cl)> tr(n, cl, 0);
  REP(i,0,n) {
    tr.update(i, 1 << (s[i] - 'a'));
  }
  REP(x,0,q) {
    int t;
    cin >> t;
    if (t == 1) {
      int i;
      char c;
      cin >> i >> c;
      i--;
      tr.update(i, 1 << (c - 'a'));
    } else {
      int l, r;
      cin >> l >> r;
      l--;
      int x = tr.accum(l, r);
      // cout << "x = " << hex << x << endl;
      int ret = 0;
      while (x > 0) {
        if (x % 2 == 1) {
          ret += 1;
        }
        x /= 2;
      }
      cout << ret << endl;
    }
  }

  return 0;
}

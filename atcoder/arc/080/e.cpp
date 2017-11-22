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

// 0-indexed
template <class T, class Op = T (*) (T, T)>
struct Segtree {
  int n;
  vector<T> dat;
  Op op;
  T unit;
  Segtree(int al, Op op, T unit): op(op), unit(unit) {
    n = 1;
    while(n < al) {
      n *= 2;
    }
    dat = vector<T>(2 * n - 1, unit);
  }
  Segtree(const vector<T> &arr, Op op, T unit): op(op), unit(unit) {
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

struct interval {
  int a, b;
  int min;
};

int main() {
  ios::sync_with_stdio(false);
  int N;
  cin >> N;
  vector<vector<PI>> as(2, vector<PI>(N/2));
  REP(i,0,N) {
    int a;
    cin >> a;
    as[i % 2][i/2] = {a, i};
  }
  auto op = [](PI x, PI y){return x.first < y.first ? x : y;};
  vector<Segtree<PI, decltype(op)>> tree;
  transform(as.begin(), as.end(), back_inserter(tree),
            [&](const vector<PI> &v){return Segtree<PI, decltype(op)>(v, op, {INT_MAX, -1});});
  auto gt = [](const interval &i1, const interval &i2){
    return i1.min > i2.min;
  };
  priority_queue<interval, vector<interval>, decltype(gt)> que(gt);
  VI ans;
  que.push({0, N, 0});
  while(!que.empty()) {
    interval iv = que.top();
    que.pop();
    PI v1 = tree[iv.a%2].accum(iv.a/2, iv.b/2);
    ans.push_back(v1.first);
    PI v2 = tree[(iv.a+1)%2].accum((v1.second+1)/2, (iv.b+1)/2);
    ans.push_back(v2.first);
    if(iv.a < v1.second) {
      PI m = tree[iv.a%2].accum(iv.a/2, v1.second/2);
      que.push({iv.a, v1.second, m.first});
    }
    if(v1.second + 1 < v2.second) {
      PI m = tree[v2.second%2].accum((v1.second+1)/2, v2.second/2);
      que.push({v1.second+1, v2.second, m.first});
    }
    if(v2.second + 1 < iv.b) {
      PI m = tree[iv.b%2].accum((v2.second+1)/2, iv.b/2);
      que.push({v2.second+1, iv.b, m.first});
    }
  }
  FOR(a,ans) {
    cout << a << " ";
  }
  cout << endl;

  return 0;
}

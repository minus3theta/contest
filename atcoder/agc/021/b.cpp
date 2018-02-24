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
#include <iomanip>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;
typedef pair<double, double> PD;

template <class T, template <class, class> class C, class charT = char>
void vdump(const C<T, allocator<T>> &v, const charT* delimiter = ", ",
           ostream &stream = cout) {
  copy(v.begin(), v.end(), ostream_iterator<T>(stream, delimiter));
  stream << endl;
}

const double pi = acos(-1.0);

struct point {
  int id;
  double x, y;
  bool operator<(const point &p) {
    if(x != p.x) return x < p.x;
    return y < p.y;
  }
  PD operator-(const point &p) const {
    return {x - p.x, y - p.y};
  }
};

ll det(const PD &p, const PD &q) {
  return p.first * q.second - p.second * q.first;
}

vector<point> convex_hull(const vector<point> &ps) {
  int n = ps.size();
  int k = 0;
  vector<point> qs(n * 2);
  REP(i,0,n) {
    while(k > 1 && det(qs[k-1] - qs[k-2], ps[i]-qs[k-1]) <= 0) {
      k--;
    }
    qs[k++] = ps[i];
  }
  for(int i = n-2, t = k; i >= 0; i--) {
    while (k > t && det(qs[k-1] - qs[k-2], ps[i] - qs[k-1]) <= 0) {
      k--;
    }
    qs[k++] = ps[i];
  }
  qs.resize(k - 1);
  return qs;
}

double dot(PD p, PD q) {
  return p.first * q.first + p.second * q.second;
}

double arg(PD p, PD q) {
  double norm = sqrt(dot(p, p) * dot(q, q));
  return acos(dot(p, q) / norm) / pi / 2.0;
}

int main() {
  ios::sync_with_stdio(false);
  int n;
  cin >> n;
  vector<point> ps(n);
  REP(i,0,n) {
    ps[i].id = i;
  }
  FOR(p,ps) {
    cin >> p.x >> p.y;
  }
  sort(ps.begin(), ps.end());
  vector<point> qs = convex_hull(ps);
  int k = qs.size();
  vector<double> ans(n, 0.0);
  REP(i,0,k) {
    int prev = (i - 1 + k) % k;
    int next = (i + 1) % k;
    PD v1 = qs[i] - qs[prev];
    PD v2 = qs[next] - qs[i];
    double t = arg(v1, v2);
    ans[qs[i].id] = t;
  }
  FOR(a,ans) {
    cout << setprecision(20) << a << endl;
  }
  return 0;
}

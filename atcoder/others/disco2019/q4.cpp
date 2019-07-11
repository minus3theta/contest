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

#define REP(m,s,n) for(int m=(int)(s);m<(int)(n);m++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

const pair<VL, VL> PROBLEM[] = {
  {
    {1, 1, 1, 1, 1, 1, 1, 1, 1, 1},
    {31, 29, 24, 21, 20, 18, 17, 14, 12, 10}
  },
  {
    {1, 1, 3, 3, 9, 9, 27, 27, 81, 81},
    {300, 200, 300, 200, 300, 250, 550, 450, 900, 800}
  },
  {
    {1, 11, 13, 17, 19, 23, 29, 31, 37, 41},
    {282, 337, 353, 404, 387, 390, 393, 316, 251, 141}
  },
  {
    {1, 3, 6, 9, 12, 15, 18, 21, 24, 27},
    {100, 151, 252, 353, 360, 371, 351, 312, 232, 134}
  },
  {
    {1, 1, 2, 4, 14, 50, 185, 512, 1111, 1234},
    {6, 4, 5, 7, 20, 59, 171, 404, 512, 618}
  },
  {
    {1, 3, 7, 13, 19, 29, 37, 43, 53, 61},
    {541, 1461, 3030, 4923, 6167, 7844, 8006, 6979, 5735, 3300}
  }
};

template <class T, template <class, class> class C, class charT = char>
void vdump(const C<T, allocator<T>> &v, const charT* delimiter = ", ",
           ostream &stream = cout) {
  copy(v.begin(), v.end(), ostream_iterator<T>(stream, delimiter));
  stream << endl;
}

const ll TASK = 10000;
const ll INF = 1e18;

ll solve(const pair<VL, VL> &machines) {
  const ll years = machines.first.size();
  VL specs = machines.first;
  REP(m, 0, years) {
    specs[m] *= (10 - m);
  }
  const VL &prices = machines.second;
  VL dp(TASK + 1, INF);
  dp[0] = 0;
  REP(m, 0, years) {
    REP(t, 0, TASK) {
      ll next_task = min(TASK, t + specs[m]);
      dp[next_task] = min(dp[next_task], dp[t] + prices[m]);
    }
  }
  return dp[TASK];
}

int main() {
  ios::sync_with_stdio(false);
  FOR(problem, PROBLEM) {
    cout << solve(problem) << endl;
  }

  return 0;
}

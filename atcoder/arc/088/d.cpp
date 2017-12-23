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

int solve(const string &s) {
  int len = s.size();
  int l, r;
  char c;
  if(len % 2 == 0) {
    r = len / 2;
    l = r - 1;
    c = s[l];
  } else {
    l = len / 2 - 1;
    r = l + 2;
    c = s[len / 2];
  }
  while(l >= 0 && s[l] == c && s[r] == c) {
    l--;
    r++;
  }
  return max(len - l - 1, (len+1) / 2);
}

int main() {
  ios::sync_with_stdio(false);
  string s;
  cin >> s;
  cout << solve(s) << endl;
  
  return 0;
}

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

struct trie {
  ll level;
  trie *cld[2];
  trie(ll level) : level(level) {
    FOR(p, cld) {
      p = NULL;
    }
  }
  ~trie() {
    FOR(p, cld) {
      if(p) {
        delete p;
      }
    }
  }
  void append(const string &s, int i) {
    int l = s.size();
    if(i >= l) {
      return;
    }
    int c = s[i] - '0';
    if(!cld[c]) {
      cld[c] = new trie(level - 1);
    }
    cld[c]->append(s, i + 1);
  }
  ll grundy() {
    if(!cld[0] && !cld[1]) {
      return 0;
    }
    ll g = 0;
    FOR(p, cld) {
      if(p) {
        g ^= p->grundy();
      } else {
        ll x = 1;
        while(level % (x * 2) == 0) {
          x *= 2;
        }
        g ^= x;
      }
    }
    return g;
  }
};

int main() {
  ios::sync_with_stdio(false);
  int n;
  ll l;
  cin >> n >> l;
  trie *t = new trie(l);
  REP(i,0,n) {
    string s;
    cin >> s;
    t->append(s, 0);
  }
  cout << (t->grundy() == 0 ? "Bob" : "Alice") << endl;
  
  delete t;
  
  return 0;
}

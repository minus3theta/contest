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

void build(deque<int> &a, deque<int> &b, ll n) {
  stack<ll> s;
  while(n != 1) {
    if(n % 2) {
      s.push(1);
      n--;
    } else {
      s.push(0);
      n /= 2;
    }
  }
  int c = 1;
  while(!s.empty()) {
    int x = s.top();
    s.pop();
    if(x) {
      a.push_back(c);
      b.push_front(c);
    } else {
      a.push_back(c);
      b.push_back(c);
    }
    c++;
  }
}

int main() {
  ios::sync_with_stdio(false);
  ll N;
  cin >> N;
  deque<int> a, b;
  build(a, b, N+1);
  cout << a.size() + b.size() << endl;
  FOR(x,a) {
    cout << x << " ";
  }
  FOR(x,b) {
    cout << x << " ";
  }
  cout << endl;
  return 0;
}

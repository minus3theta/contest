#include <cstdio>
#include <climits>
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

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

int N;
VI bit(1<<20, 0);
VI a(200001);

void add(int i, int x) {
  i++;
  while(i<=N) {
    bit[i] += x;
    i += i & -i;
  }
}

int sum(int i) {
  i++;
  int s = 0;
  while(i > 0) {
    s += bit[i];
    i -= i & -i;
  }
  return s;
}

PI score(int op) {
  int maxs = 0;
  int maxi = -1;
  int s = sum(op);
  REP(i,op+1,N) {
    if(maxs < a[i] - (sum(i-1) - s)) {
      maxs = a[i] - (sum(i-1) - s);
      maxi = i;
    }
  }
  return {maxi,maxs};
}

int main() {
  cin >> N;
  REP(i,0,N-1) {
    cin >> a[i];
  }
  int M;
  cin >> M;
  if(M != 1) return 0;
  cin >> a[N-1];
  REP(i,0,N) {
    add(i, a[i]);
  }
  int x = 0, y = 1;
  int ds = 0;
  while(x < N-1 || y < N-1) {
    if(x < y) {
      auto p = score(y);
      ds += p.second;
      x = p.first;
      y = x - 1;
    } else {
      auto p = score(x);
      ds -= p.second;
      y = p.first;
      x = y - 1;
    }
  }
  cout << ds << endl;
  return 0;
}

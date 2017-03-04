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
#include <cstdlib>

#define REP(i,s,n) for(int i=(int)(s);i<(int)(n);i++)
#define FOR(x,xs) for(auto &x: xs)

using namespace std;
typedef long long ll;
typedef pair<int,int> PI;
typedef pair<ll,ll> PL;
typedef vector<int> VI;
typedef vector<ll> VL;

struct feed {
  int r, c, f, d;
};

const char DIRS[] = {'U', 'R', 'D', 'L'};

int main() {
  ios::sync_with_stdio(false);
  int H, W, K;
  PI start;
  cin >> H >> W >> K;
  vector<string> field(H);
  FOR(s,field) {
    cin >> s;
  }
  int N;
  cin >> N;
  vector<feed> fs(N);
  FOR(f,fs) {
    cin >> f.r >> f.c >> f.f >> f.d;
  }
  REP(i,0,K) {
    cout << DIRS[rand()%4];
  }
  cout << endl;
  return 0;
}

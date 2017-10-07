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

ll score_nsew(ll A, ll B, vector<vector<bool>> &m, int H, int W) {
  int stones = 0;
  REP(i,0,H/2) {
    REP(j,0,W/2) {
      if(m[i][j]) {
        stones++;
      }
    }
  }
  return (A + B + max(A, B)) * stones;
}

ll score_ns(ll A, ll B, vector<vector<bool>> &m, int H, int W) {
  ll score = 0;
  bool rm = false;
  REP(i,0,H/2) {
    REP(j,0,W) {
      if(m[i][j] != m[H-i-1][j]) {
        m[i][j] = m[H-i-1][j] = false;
        rm = true;
      }
    }
  }
  if(rm) {
    score += A;
  }
  int ch = 0;
  REP(i,0,H/2) {
    REP(j,0,W/2) {
      if(m[i][j] != m[i][W-j-1]) {
        ch++;
        m[i][j] = m[i][W-j-1] = false;
      }
    }
  }
  score += A * ch + B;
  return score + score_nsew(A, B, m, H, W);
}

ll score_ew(ll A, ll B, vector<vector<bool>> &m, int H, int W) {
  ll score = 0;
  bool rm = false;
  REP(i,0,H) {
    REP(j,0,W/2) {
      if(m[i][j] != m[i][W-j-1]) {
        m[i][j] = m[i][W-j-1] = false;
        rm = true;
      }
    }
  }
  if(rm) {
    score += B;
  }
  int ch = 0;
  REP(i,0,H/2) {
    REP(j,0,W/2) {
      if(m[i][j] != m[H-i-1][j]) {
        ch++;
        m[i][j] = m[H-i-1][j] = false;
      }
    }
  }
  score += B * ch + A;
  return score + score_nsew(A, B, m, H, W);
}

bool is_nsew(vector<vector<bool>> &m, int H, int W) {
  REP(i,0,H/2) {
    REP(j,0,W/2) {
      if(!(m[i][j] == m[i][W-j-1] && m[i][j] == m[H-i-1][W-j-1] && m[i][j] == m[H-i-1][j])) {
        return false;
      }
    }
  }
  return true;
}

int main() {
  ios::sync_with_stdio(false);
  int H, W;
  ll A, B;
  cin >> H >> W >> A >> B;
  vector<vector<bool>> m_ns(H, vector<bool>(W));
  vector<vector<bool>> m_ew(H, vector<bool>(W));
  REP(i,0,H) {
    string s;
    cin >> s;
    REP(j,0,W) {
      m_ns[i][j] = m_ew[i][j] = s[j] == 'S';
    }
  }
  if(is_nsew(m_ns, H, W)) {
    cout << score_nsew(A, B, m_ns, H, W) << endl;
  } else {
    ll score1 = score_ns(A, B, m_ns, H, W);
    ll score2 = score_ew(A, B, m_ew, H, W);
    cout << max(score1, score2) << endl;
  }
  
  return 0;
}

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

const vector<string> FIELD[] = {
  {
    "00BBBB0F",
    "000BBBFF",
    "000D0EE0",
    "DDDD*EE0",
    "AAAACCCC",
    "AAAA0C0C",
    "001111G0",
    "001111G0",
  },
  {
    "*0B0BBFF",
    "00BBBBF0",
    "0D000000",
    "DDDD*000",
    "CCCCAAAA",
    "0C0CAAAA",
    "G01111EE",
    "G01111EE",
  },
  {
    "*EEBBBB0",
    "0EE0BBBF",
    "AAAA00FF",
    "AAAA*0*0",
    "CCCC0D00",
    "00CCDDDD",
    "00111100",
    "0*1111GG",
  },
  {
    "F0AAAA00",
    "FFAAAA00",
    "0D00000*",
    "DDDD*000",
    "CCCCB0BB",
    "0C0CBBBB",
    "G01111EE",
    "G01111EE",
  },
  {
    "FFAAAA00",
    "F0AAAAEE",
    "00D000EE",
    "DDDD***0",
    "BBBBCCC0",
    "0BBB0CCC",
    "0G111100",
    "*G1111*0",
  },
  {
    "*000AAAA",
    "FFF0AAAA",
    "DDD0EE0*",
    "00DD*EE0",
    "0CCCBBBB",
    "CC0C0BBB",
    "*01111GG",
    "001111*0",
  },
};

constexpr int SIZE = 8;
constexpr int BLOCK = 7;

pair<PI, PI> bounding_box(const vector<string> &field, char c) {
  // ((top, left), (height, width))
  int top = SIZE;
  int bottom = 0;
  int left = SIZE;
  int right = 0;
  REP(i,0,SIZE) {
    REP(j,0,SIZE) {
      if (field[i][j] == c) {
        top = min(top, i);
        bottom = max(bottom, i);
        left = min(left, j);
        right = max(right, j);
      }
    }
  }
  return {{top, left}, {bottom - top + 1, right - left + 1}};
}

struct block {
  char c;
  int height, width;
  vector<vector<bool>> shape;
};

bool pole(const vector<string> &current_field, const vector<block> &block_data, int k, PI pos) {
  const block &b = block_data[k];
  REP(i,0,b.height) {
    REP(j,0,b.width) {
      char c = current_field[i+pos.first][j+pos.second];
      if (b.shape[i][j] && c == '*') {
        return true;
      }
    }
  }
  return false;
}

bool invalid_move(const vector<string> &current_field, const vector<block> &block_data, int k, PI pos) {
  const block &b = block_data[k];
  REP(i,0,b.height) {
    REP(j,0,b.width) {
      char c = current_field[i+pos.first][j+pos.second];
      if (b.shape[i][j] && c != b.c && c != '0') {
        return true;
      }
    }
  }
  return false;
}

int solve(const vector<string> &field) {
  vector<PI> init;
  vector<block> block_data;
  REP(k,0,BLOCK) {
    auto bb = bounding_box(field, 'A' + k);
    PI pos = bb.first;
    init.push_back(pos);
    PI size = bb.second;
    block b = {
      (char)('A' + k),
      size.first,
      size.second,
      vector<vector<bool>>(size.first, vector<bool>(size.second, false)),
    };
    REP(i,0,size.first) {
      REP(j,0,size.second) {
        if (field[i+pos.first][j+pos.second] == b.c) {
          b.shape[i][j] = true;
        }
      }
    }
    block_data.push_back(b);
  }
  PI goal = bounding_box(field, '1').first;
  map<vector<PI>, int> dist;
  dist[init] = 0;
  queue<vector<PI>> q;
  q.push(init);
  while (! q.empty()) {
    auto state = q.front();
    q.pop();
    int d = dist[state];
    if (state[0] == goal) {
      return d;
    }
    vector<string> current_field(SIZE, "00000000");
    REP(i,0,SIZE) {
      REP(j,0,SIZE) {
        if (field[i][j] == '*') {
          current_field[i][j] = '*';
        }
      }
    }
    REP(b,0,BLOCK) {
      PI pos = state[b];
      REP(i,0,block_data[b].height) {
        REP(j,0,block_data[b].width) {
          if (block_data[b].shape[i][j]) {
            current_field[pos.first + i][pos.second + j] = block_data[b].c;
          }
        }
      }
    }
    REP(k,0,BLOCK) {
      block &b = block_data[k];
      PI current_pos = state[k];
      // Move forward
      for(int i=current_pos.first-1; i>=0; i--) {
        PI pos = state[k];
        pos.first = i;
        vector<PI> new_state = state;
        new_state[k] = pos;
        if (dist.find(new_state) != dist.end()) {
          continue;
        }
        if (pole(current_field, block_data, k, pos)) {
          break;
        }
        if (invalid_move(current_field, block_data, k, pos)) {
          continue;
        }
        dist[new_state] = d + 1;
        q.push(new_state);
      }
      // Move backward
      REP(i,current_pos.first + 1,SIZE-b.height+1) {
        PI pos = state[k];
        pos.first = i;
        vector<PI> new_state = state;
        new_state[k] = pos;
        if (dist.find(new_state) != dist.end()) {
          continue;
        }
        if (pole(current_field, block_data, k, pos)) {
          break;
        }
        if (invalid_move(current_field, block_data, k, pos)) {
          continue;
        }
        dist[new_state] = d + 1;
        q.push(new_state);
      }
      // Move left
      for(int j=current_pos.second-1; j>=0; j--) {
        PI pos = state[k];
        pos.second = j;
        vector<PI> new_state = state;
        new_state[k] = pos;
        if (dist.find(new_state) != dist.end()) {
          continue;
        }
        if (pole(current_field, block_data, k, pos)) {
          break;
        }
        if (invalid_move(current_field, block_data, k, pos)) {
          continue;
        }
        dist[new_state] = d + 1;
        q.push(new_state);
      }
      // Move right
      REP(j,current_pos.second+1,SIZE-b.width+1) {
        PI pos = state[k];
        pos.second = j;
        vector<PI> new_state = state;
        new_state[k] = pos;
        if (dist.find(new_state) != dist.end()) {
          continue;
        }
        if (pole(current_field, block_data, k, pos)) {
          break;
        }
        if (invalid_move(current_field, block_data, k, pos)) {
          continue;
        }
        dist[new_state] = d + 1;
        q.push(new_state);
      }
    }
  }

  return -1;
}

int main() {
  ios::sync_with_stdio(false);
  FOR(f,FIELD) {
    cout << solve(f) << endl;
  }

  return 0;
}

#include <iostream>
#include <string>
#include <cstdio>

using namespace std;

int main() {
  string hand[3];
  int idx[] = {0, 0, 0};
  for(int i=0; i<3; i++) {
    cin >> hand[i];
  }
  int turn = 0;
  while(true) {
    if(idx[turn] >= hand[turn].size()) {
      printf("%c\n", 'A' + turn);
      break;
    }
    turn = hand[turn][idx[turn]++] - 'a';
  }
  return 0;
}

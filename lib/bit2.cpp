// 2-dimensional BIT
// 1-indexed
struct Bit2 {
  int n;
  vector<VI> dat;
  Bit(int n) : n(n), dat(n + 1, VI(n + 1, 0)) {
  }
  void operate(int k, int l0, int a) {
    while(k <= n) {
      int l = l0;
      while(l <= n) {
        dat[k][l] += a;
        l += l & -l;
      }
      k += k & -k;
    }
  }
  int accum(int k, int l0) {
    int sum = 0;
    while(k > 0) {
      int l = l0;
      while(l > 0) {
        sum += dat[k][l];
        l -= l & -l;
      }
      k -= k & -k;
    }
    return sum;
  }
};

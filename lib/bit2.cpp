// 2-dimensional BIT
// 1-indexed
template <class T, class Op, T unit>
struct Bit2 {
  int m, n;
  vector<vector<T> > dat;
  Op op;
  Bit2(int m, int n) : m(m), n(n), dat(m + 1, vector<T>(n + 1, unit)) {
  }
  void operate(int k0, int l0, T a) {
    for(int k=k0; k <= m; k += k&-k) {
      for(int l=l0; l <= n; l += l&-l) {
        dat[k][l] = op(dat[k][l], a);
      }
    }
  }
  T accum(int k0, int l0) {
    T sum = unit;
    for(int k=k0; k > 0; k -= k&-k) {
      for(int l=l0; l > 0; l -= l&-l) {
        sum = op(sum, dat[k][l]);
      }
    }
    return sum;
  }
};

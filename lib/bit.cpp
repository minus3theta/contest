// 1-indexed
template <class T, class Op, T unit>
struct Bit {
  int n;
  vector<T> dat;
  Op op;
  Bit(int n) : n(n), dat(n + 1, unit) {
  }
  Bit(const vector<T> &arr) : n(arr.size()), dat(arr.size() + 1) {
    copy(arr.begin(), arr.end(), dat.begin() + 1);
    for(int i=1; i<n; i++) {
      dat[i + (i & -i)] = op(dat[i + (i & -i)], dat[i]);
    }
  }
  void operate(int k, T a) {
    while(k <= n) {
      dat[k] = op(dat[k], a);
      k += k & -k;
    }
  }
  T accum(int k) {
    T sum = unit;
    while(k > 0) {
      sum = op(sum, dat[k]);
      k -= k & -k;
    }
    return sum;
  }
};

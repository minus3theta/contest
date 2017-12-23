// 1-indexed
template <class T, class Op = T (*) (T, T)>
struct Bit {
  int n;
  vector<T> dat;
  Op op;
  T unit;
  Bit(int n, Op op = Op(), T unit = T()) : n(n), dat(n + 1, unit), op(op), unit(unit) {
  }
  Bit(const vector<T> &arr, Op op = Op(), T unit = T())
    : n(arr.size()), dat(arr.size() + 1), op(op), unit(unit) {
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

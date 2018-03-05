template <class T = double>
struct coord {
  T x, y;
  coord<T> operator-() const {return {-x, -y};}
  T norm_sq() const {return x * x + y * y;}
  coord<T> operator+(const coord &p) const {return {x + p.x, y + p.y};}
  coord<T> operator-(const coord &p) const {return {x - p.x, y - p.y};}
  T operator*(const coord &p) const {return x * p.x + y * p.y;}
  T det(const coord &p) const {return x * p.y - y * p.x;}
  bool operator<(const coord &p) const {
    if(x != p.x) return x < p.x;
    return y < p.y;
  }
};

template <class T = double>
struct point {
  int id;
  coord<T> cd;
  bool operator<(const point &p) const {
    return cd < p.cd;
  }
};

vector<point<>> convex_hull(const vector<point<>> &ps) {
  int n = ps.size();
  int k = 0;
  vector<point<>> qs(n * 2);
  REP(i,0,n) {
    while(k > 1 && (qs[k-1].cd-qs[k-2].cd).det(ps[i].cd-qs[k-1].cd) <= 0) {
      k--;
    }
    qs[k++] = ps[i];
  }
  for(int i = n-2, t = k; i >= 0; i--) {
    while (k > t && (qs[k-1].cd-qs[k-2].cd).det(ps[i].cd-qs[k-1].cd) <= 0) {
      k--;
    }
    qs[k++] = ps[i];
  }
  qs.resize(k - 1);
  return qs;
}

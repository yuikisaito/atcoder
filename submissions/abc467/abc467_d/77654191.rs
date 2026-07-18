use nalgebra::Vector2;
use proconio::input;

fn main() {
  input! {
      t: usize,
  }

  for _ in 0..t {
    input! {
        p: (f64, f64),
        q: (f64, f64),
        r: (f64, f64),
        s: (f64, f64),
    }

    let p = Vector2::new(p.0, p.1);
    let q = Vector2::new(q.0, q.1);
    let r = Vector2::new(r.0, r.1);
    let s = Vector2::new(s.0, s.1);

    let pq = q - p;
    let rs = s - r;

    let parallel = pq.x * rs.y == pq.y * rs.x;

    let d1 = (p + q - r * 2.).norm_squared();
    let d2 = (p + q - s * 2.).norm_squared();

    println!("{}", if parallel && d1 != d2 { "No" } else { "Yes" });
  }
}

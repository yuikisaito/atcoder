use nalgebra::Vector2;
use proconio::input;

fn main() {
  input! {
      t: usize,
  }

  for _ in 0..t {
    input! {
        p: (isize, isize),
        q: (isize, isize),
        r: (isize, isize),
        s: (isize, isize),
    }

    let p = Vector2::new(p.0, p.1);
    let q = Vector2::new(q.0, q.1);
    let r = Vector2::new(r.0, r.1);
    let s = Vector2::new(s.0, s.1);

    let pq = q - p;
    let rs = s - r;

    let parallel = pq.x * rs.y == pq.y * rs.x;

    let v1 = p + q - r * 2;
    let d1 = v1.dot(&v1);
    let v2 = p + q - s * 2;
    let d2 = v2.dot(&v2);

    println!("{}", if parallel && d1 != d2 { "No" } else { "Yes" });
  }
}

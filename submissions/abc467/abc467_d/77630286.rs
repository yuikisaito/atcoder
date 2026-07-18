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

    println!(
      "{}",
      if (q.0 - p.0) * (s.1 - r.1) == (q.1 - p.1) * (s.0 - r.0) && (p.0 + q.0 - 2 * r.0).pow(2) + (p.1 + q.1 - 2 * r.1).pow(2) != (p.0 + q.0 - 2 * s.0).pow(2) + (p.1 + q.1 - 2 * s.1).pow(2) {
        "No"
      } else {
        "Yes"
      }
    )
  }
}

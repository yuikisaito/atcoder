use proconio::input_interactive;

fn main() {
  input_interactive! {
    t: usize,
  }

  for _ in 0..t {
    input_interactive! {
      p: (isize, isize),
      q: (isize, isize),
      r: (isize, isize),
      s: (isize, isize),
    }

    println!("{}", if (q.0 - p.0) * (s.1 - r.1) == (q.1 - p.1) * (s.0 - r.0) { "No" } else { "Yes" })
  }
}

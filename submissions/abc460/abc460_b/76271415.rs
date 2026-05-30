use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    t: usize,
    cases: [(usize, usize, usize, usize, usize, usize); t],
  }

  println!(
    "{}",
    cases
      .into_iter()
      .map(|(x1, y1, r1, x2, y2, r2)| {
        let d = x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2);
        if d <= (r1 + r2).pow(2) && d >= r1.abs_diff(r2).pow(2) {
          "Yes"
        } else {
          "No"
        }
      })
      .join("\n")
  );
}

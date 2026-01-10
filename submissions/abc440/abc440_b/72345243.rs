use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    t: [usize; n],
  }

  println!(
    "{}",
    t.into_iter()
      .enumerate()
      .sorted_by_key(|x| x.1)
      .map(|(x, _y)| x + 1)
      .take(3)
      .join(" ")
  );
}

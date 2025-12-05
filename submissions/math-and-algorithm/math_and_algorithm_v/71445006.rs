use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
      n: usize,
      a: [usize; n],
  }

  println!(
    "{}",
    a.into_iter()
      .combinations(2)
      .filter(|x| x[0] + x[1] == 100000)
      .count()
  );
}

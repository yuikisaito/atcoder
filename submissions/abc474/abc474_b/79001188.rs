use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
  input! {
    n: usize,
    p: [Usize1; n],
  }

  println!("{}", if p.into_iter().chunks(10).into_iter().enumerate().all(|(i, mut a)| a.all(|j| 10 * i <= j && j < 10 * (i + 1))) { "Yes" } else { "No" });
}

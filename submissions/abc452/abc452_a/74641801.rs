use std::collections::HashSet;

use proconio::input;

fn main() {
  input! {
    m: usize,
    d: usize,
  }

  let dates = HashSet::from([(1, 7), (3, 3), (5, 5), (7, 7), (9, 9)]);
  println!("{}", if dates.contains(&(m, d)) { "Yes" } else { "No" });
}

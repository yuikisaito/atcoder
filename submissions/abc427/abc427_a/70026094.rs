use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  println!(
    "{}",
    s.iter()
      .enumerate()
      .filter(|&(i, _)| i != s.len() / 2)
      .map(|(_, &c)| c)
      .join("")
  );
}

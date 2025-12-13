use std::iter::repeat;

use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    s: Chars,
  }

  println!(
    "{}",
    repeat('o')
      .take(n - s.len())
      .chain(s.into_iter())
      .collect::<String>()
  );
}

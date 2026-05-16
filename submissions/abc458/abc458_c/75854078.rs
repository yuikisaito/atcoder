use std::cmp::min;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    s: Chars,
  }

  let l = s.len();

  println!("{:?}", s.into_iter().positions(|c| c == 'C').map(|p| min(p + 1, l - p)).sum::<usize>());
}

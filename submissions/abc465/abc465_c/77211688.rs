use std::collections::VecDeque;

use itertools::izip;
use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    s: Chars,
  }

  let mut chars = VecDeque::new();
  let mut from_left = true;

  for (reversed, n) in izip!(s.into_iter().map(|c| c == 'o'), 1..) {
    eprintln!("{}", from_left);
    if !from_left {
      chars.push_front(n.to_string());
    } else {
      chars.push_back(n.to_string());
    }
    from_left = from_left == !reversed;
  }

  println!("{}", if from_left { chars.into_iter().join(" ") } else { chars.into_iter().rev().join(" ") });
}

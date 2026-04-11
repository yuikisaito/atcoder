use std::isize;

use itertools::izip;
use proconio::input;

fn main() {
  input! {
    t: usize,
    x: usize,
    a: [isize; t + 1],
  }

  let mut prev = isize::MIN;
  for (i, now) in izip!(a, 0..=t) {
    if i.abs_diff(prev) >= x {
      println!("{} {}", now, i);
      prev = i;
    }
  }
}

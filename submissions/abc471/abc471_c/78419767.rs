use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [isize; n],
  }

  let (x, y): (Vec<_>, Vec<_>) = a.into_iter().partition(|i| *i < 0);
  let mut left = BinaryHeap::from(x);
  let mut right = BinaryHeap::from_iter(y.into_iter().map(|i| Reverse(i)));

  let mut center = 0;
  let mut ans = 0;
  for _ in 0..n {
    let l = left.peek().map_or_else(|| isize::MAX, |i| -(i - center));
    let r = right.peek().map_or_else(|| isize::MAX, |i| i.0 - center);
    if l <= r {
      center = left.pop().unwrap();
      ans += l;
    } else {
      Reverse(center) = right.pop().unwrap();
      ans += r;
    }
    while let Some(l) = left.peek()
      && *l > center
    {
      right.push(Reverse(left.pop().unwrap()));
    }
    while let Some(Reverse(r)) = right.peek()
      && *r < center
    {
      left.push(right.pop().unwrap().0);
    }
  }

  println!("{}", ans);
}

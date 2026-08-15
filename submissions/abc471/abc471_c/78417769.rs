use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [i64; n],
  }

  let (x, y): (Vec<_>, Vec<_>) = a.into_iter().partition(|i| *i < 0);
  let mut left = BinaryHeap::from(x);
  let mut right = BinaryHeap::from_iter(y.into_iter().map(|i| Reverse(i)));

  let mut center = 0;
  let mut ans = 0;
  for _ in 0..n {
    let l = left.peek().unwrap_or(&(i32::MIN as i64));
    let r = right.peek().map_or_else(|| &(i32::MAX as i64), |i| &i.0);
    let l_distance = -(l - center);
    let r_distance = r - center;
    if l_distance <= r_distance {
      center = left.pop().unwrap();
      ans += l_distance;
    } else {
      Reverse(center) = right.pop().unwrap();
      ans += r_distance;
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

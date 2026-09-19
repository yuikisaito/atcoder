use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
  input! {
    n: usize,
    mut a: [usize; n],
  }

  let remaining = a.split_off(3);

  let mut x = BinaryHeap::from_iter(a.into_iter().map(Reverse));
  let mut y = BinaryHeap::new();

  remaining.into_iter().for_each(|i| {
    println!("{}", x.peek().unwrap().0);
    if x.peek().unwrap().0 <= i {
      x.push(Reverse(i));
      y.push(x.pop().unwrap().0);
    } else {
      y.push(i);
    }
  });
  println!("{}", x.peek().unwrap().0);
}

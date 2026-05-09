use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
  input! {
    n: usize,
    t: usize,
    a: [usize; n],
  }

  let mut map: BinaryHeap<_> = a.into_iter().zip(1..).map(Reverse).collect();
  for _ in 0..t {
    let mut val = map.peek_mut().unwrap();
    let (k, v) = val.0;
    *val = Reverse((k + v, v));
  }
  println!("{}", map.into_iter().map(|x| x.0 .0).min().unwrap());
}

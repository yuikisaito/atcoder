use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    l: [usize; n],
  }

  let kirikomi_pos = l
    .into_iter()
    .scan(0, |state, i| {
      *state += i;
      Some(*state)
    })
    .collect_vec();
  let len = kirikomi_pos[n - 1];
  let mut bh = BinaryHeap::from(kirikomi_pos);
  let mut prev = usize::MAX;
  while let Some(i) = bh.pop() {
    let d = len.abs_diff(i * 2);
    if prev < d {
      println!("{}", prev);
      return;
    }
    prev = d;
  }
  println!("{}", prev);
}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    xy: [(Usize1, Usize1); n],
  }

  let mut bh = BinaryHeap::from(xy.into_iter().map(Reverse).collect_vec());
  let mut min = n;
  let mut ans = 0;
  for _ in 0..n {
    let Reverse((_, y)) = bh.pop().unwrap();
    if y < min {
      min = y;
      ans += 1;
    }
  }

  println!("{}", ans);
}

use std::collections::BinaryHeap;

use itertools::Itertools;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
  input! {
    n: usize,
    d: usize,
    x: [usize; n],
  }

  let mut bh = BinaryHeap::from_iter(x.into_iter().enumerate().map(|(i, j)| (j, i + 1)));
  let first = bh.pop().unwrap();
  let mut prev_pos = first.0;
  let mut prev_index = first.1;
  let mut prev_kakusu = true;
  let mut ans = Vec::new();

  while let Some((i, p)) = bh.pop() {
    eprintln!("{} {} {}", prev_index, prev_kakusu, prev_pos);
    let kakusu = prev_pos - i >= d;
    if prev_kakusu && kakusu {
      ans.push(prev_index);
    }
    prev_pos = i;
    prev_index = p;
    prev_kakusu = kakusu;
  }
  if prev_kakusu {
    ans.push(prev_index);
  }

  println!("{}", ans.len());
  println!("{}", ans.into_iter().sorted().join(" "));
}

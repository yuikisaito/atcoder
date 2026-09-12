use std::cmp::Ordering::Less;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    s: Usize1,
    l: u64,
    mut a: [u64; n-1],
  }

  let right = a.split_off(s).into_iter().rev().collect_vec();
  let left = a;
  println!(
    "{}",
    (0..=1)
      .map(|b| {
        let (mut x, mut y);
        if b == 0 {
          x = left.iter().map(|i| i * 2).collect_vec();
          y = right.clone();
        } else {
          x = left.clone();
          y = right.iter().map(|i| i * 2).collect_vec();
        }

        let mut sum = 0;
        let mut cnt = 1;
        while sum < l && !(x.is_empty() && y.is_empty()) {
          sum += if x.last().unwrap_or(&u64::MAX).cmp(y.last().unwrap_or(&u64::MAX)) == Less { x.pop().unwrap() } else { y.pop().unwrap() };
          cnt += 1;
        }
        if sum > l {
          cnt -= 1;
        }
        cnt
      })
      .max()
      .unwrap()
  );
}

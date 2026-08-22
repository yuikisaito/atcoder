use std::collections::VecDeque;

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
  input! {
    n: usize,
    m: usize,
    k: u64,
    a: [u64; n],
  }

  let mut remove_reservation: VecDeque<_> = vec![0; m].into();
  let mut remaining = k;
  for i in 0..n {
    remaining += remove_reservation.pop_front().unwrap();
    if remaining >= a[i] {
      remaining -= a[i];
      remove_reservation.push_back(a[i]);
      println!("Yes");
    } else {
      remove_reservation.push_back(0);
      println!("No");
    }
  }
}

use std::collections::BinaryHeap;

use either::Either::Left;
use either::Either::Right;
use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    t: usize,
  }

  for _ in 0..t {
    input! {
      n: usize,
      ab: [(isize, isize); n],
    }

    let coupon = ab.clone().into_iter().min_by_key(|(a, _)| *a).map(|(a, _)| a).unwrap();
    let mut waribiki: BinaryHeap<_> = ab.clone().into_iter().map(|(a, b)| a - b).collect();
    let (with_coupon, for_coupon): (Vec<_>, Vec<_>) = ab.into_iter().partition_map(|(a, b)| if b + coupon < a { Left(b + coupon) } else { Right(a) });
    let mut bought = for_coupon.len();
    let mut dup = 0;
    dup += with_coupon.len().min(for_coupon.len()) as isize * -coupon;
    bought -= with_coupon.len().min(for_coupon.len());
    dup += -(0..bought / 2).map(|_| waribiki.pop().unwrap()).sum::<isize>();
    eprintln!("{:?} {:?} {:?}", with_coupon, for_coupon, dup);
    let ans = for_coupon.into_iter().chain(with_coupon.into_iter()).sum::<isize>() + dup;
    println!("{}", ans);
  }
}

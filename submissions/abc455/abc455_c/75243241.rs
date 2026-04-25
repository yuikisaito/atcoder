use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    a: [usize; n],
  }

  let counts: BTreeMap<usize, usize> = a.into_iter().counts().into_iter().collect();
  println!("{:?}", counts.into_iter().rev().skip(k).map(|(k, v)| k * v).sum::<usize>());
}

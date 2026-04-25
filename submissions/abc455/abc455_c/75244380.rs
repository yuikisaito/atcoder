use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    k: usize,
    a: [usize; n],
  }

  let sums = a.into_iter().counts().into_iter().map(|(k, v)| k * v).sorted().collect_vec();
  println!("{:?}", sums.into_iter().rev().skip(k).sum::<usize>());
}

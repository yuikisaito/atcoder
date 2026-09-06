use itertools::{Itertools, izip};
use num::Signed;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [u64; n],
    b: [u64; n],
  }

  let diffs = izip!(a, b).map(|(i, j)| i as isize - j as isize).collect_vec();
  let (pos, neg): (Vec<_>, Vec<_>) = diffs.clone().into_iter().partition(|i| i.is_positive());
  let pos_sum = pos.into_iter().sum::<isize>() as u64;
  let neg_sum = neg.into_iter().map(|i| -i).sum::<isize>() as u64;

  if pos_sum * 10u64.pow(18) <= neg_sum * 1 {
    println!("No");
  } else {
    println!("Yes");
    println!("{}", diffs.iter().map(|i| if i.is_positive() { 10u64.pow(18) } else { 1 }).join(" "));
  }
}

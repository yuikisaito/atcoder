use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: u64,
    k: u64,
    x: u64,
    a: [u64; n],
  }

  let mut sum = 0;
  let mut i = 0;
  let mut sorted = a.into_iter().sorted();
  while sum < x && i < k {
    sum += sorted.next().unwrap();
    i += 1;
  }

  if sum < x {
    println!("-1");
  } else {
    println!("{}", i + k - n);
  }
}

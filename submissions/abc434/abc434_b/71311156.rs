use std::collections::HashMap;

use proconio::input;

fn main() {
  input! {
    n: usize,
    _m: usize,
    ab: [(usize, usize); n],
  }

  let mut sum = HashMap::new();
  let mut len = HashMap::new();
  for (a, b) in ab {
    *sum.entry(a).or_insert(0) += b;
    *len.entry(a).or_insert(0) += 1;
  }
  let sorted_sum: Vec<_> = sum.iter().collect();
  for (a, b) in sorted_sum {
    println!("{}", *b as f64 / *len.get(&a).unwrap() as f64)
  }
}

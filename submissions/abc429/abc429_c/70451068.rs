use std::collections::HashMap;

use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut nums = HashMap::new();
  for k in a {
    if let Some(v) = nums.get(&k) {
      nums.insert(k, v + 1);
    } else {
      nums.insert(k, 1usize);
    }
  }

  let keys = nums.keys();
  let mut two_from_two = nums.clone();
  for &k in keys {
    let v = *nums.get(&k).unwrap();
    if v >= 2 {
      two_from_two.insert(k, v * (v - 1) / 2);
    } else {
      two_from_two.remove(&k);
    }
  }

  let keys_len = nums.keys().len();
  if keys_len < 2 {
    println!("0");
    return;
  }

  let two_cnt = two_from_two.len();
  let one_cnt = keys_len - two_cnt;

  let mut one_two = 0;
  for (_, v) in two_from_two.clone() {
    one_two += v * one_cnt;
  }

  let mut two_two = 0;
  for (_, v) in two_from_two {
    two_two += v * (two_cnt - 1);
  }

  println!("{}", one_two + two_two);
}

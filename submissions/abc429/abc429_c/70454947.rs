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

  let mut two_from_two = nums.clone();
  for &k in nums.keys() {
    let v = *nums.get(&k).unwrap();
    if v >= 2 {
      two_from_two.insert(k, v * (v - 1) / 2);
    } else {
      two_from_two.remove(&k);
    }
  }

  // let mut one_from_two = nums.clone();

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
  for ka in two_from_two.keys() {
    for kb in two_from_two.keys() {
      if ka != kb {
        two_two += two_from_two.get(ka).unwrap() * nums.get(kb).unwrap();
      }
    }
  }

  println!("{}", one_two + two_two);
}

use std::collections::BTreeMap;

use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut first = BTreeMap::new();

  for (x, i) in a.into_iter().zip(0..) {
    if !first.contains_key(&x) {
      first.insert(x, i);
    }
  }

  let mut longest = 0;
  let mut before_k = first.first_key_value().unwrap().0 - 1;
  let mut before_v = -1;
  let mut crnt = 0;

  for (k, v) in first {
    if before_v < v && before_k + 1 == k {
      crnt += 1;
    } else {
      longest = longest.max(crnt);
    }

    before_v = v;
    before_k = k;
  }
  longest = longest.max(crnt);

  println!("{}", longest);
}

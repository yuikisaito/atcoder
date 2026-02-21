use std::collections::BTreeMap;

use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  let mut pair: BTreeMap<usize, (isize, isize)> = BTreeMap::new();

  for (x, i) in a.into_iter().zip(0..) {
    if let Some(crnt) = pair.get(&x) {
      pair.insert(x, (crnt.0, i));
    } else {
      pair.insert(x, (i, i));
    }
  }

  let mut longest = 0;
  let mut before_k = pair.first_key_value().unwrap().0 - 1;
  let mut before_v = -1;
  let mut crnt = 0;

  for (k, v) in pair {
    if before_v < v.1 && before_k + 1 == k {
      crnt += 1;
    } else {
      longest = longest.max(crnt);
    }

    before_v = v.0;
    before_k = k;
  }
  longest = longest.max(crnt);

  println!("{}", longest);
}

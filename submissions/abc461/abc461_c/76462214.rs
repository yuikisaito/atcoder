use std::collections::BinaryHeap;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    k: usize,
    m: usize,
    cv: [(Usize1, usize); n],
  }

  let mut value = 0;
  let size = cv.iter().map(|(c, _)| c).max().unwrap();

  let mut color = vec![0; *size + 1];
  let mut others = BinaryHeap::new();

  for (c, v) in cv {
    if color[c] < v {
      others.push(color[c]);
      color[c] = v;
    } else {
      others.push(v);
    }
  }

  let mut max_color = BinaryHeap::from(color);
  for _ in 0..m {
    value += max_color.pop().unwrap();
  }

  for i in max_color {
    others.push(i);
  }

  for _ in 0..k - m {
    value += others.pop().unwrap();
  }

  println!("{}", value);
}

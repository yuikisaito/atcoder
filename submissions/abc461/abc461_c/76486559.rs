use std::collections::BinaryHeap;
use std::mem::replace;

use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    k: usize,
    m: usize,
    cv: [(Usize1, usize); n],
  }

  let mut max_per_color = Vec::new();
  let mut pool = BinaryHeap::new();

  for (c, v) in cv {
    if c >= max_per_color.len() {
      max_per_color.resize(c + 1, 0);
    }

    if v > max_per_color[c] {
      pool.push(replace(&mut max_per_color[c], v));
    } else {
      pool.push(v);
    }
  }

  let mut chosen = BinaryHeap::from(max_per_color);
  let mut ans = 0;

  for _ in 0..m {
    ans += chosen.pop().unwrap();
  }

  pool.extend(chosen);

  for _ in 0..k - m {
    ans += pool.pop().unwrap();
  }

  println!("{}", ans);
}

use std::collections::BinaryHeap;

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
  input! {
    q: usize,
    v: isize,
  }

  let mut batteries = BinaryHeap::new();

  for _ in 0..q {
    input! {
      t: usize
    }

    match t {
      1 => {
        input! {
          t: isize,
          w: isize,
        }

        batteries.push(w - t);
      },
      2 => {
        input! {
          t: isize,
        }
        println!("{}", if let Some(ejected) = batteries.pop() { v.min(ejected + t) } else { -1 })
      },
      _ => unreachable!(),
    }
  }
}

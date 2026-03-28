use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn main() {
  input! {
      q: usize,
      queries: [(usize, usize); q],
  }

  let mut heap: BinaryHeap<Reverse<usize>> = BinaryHeap::new();

  for (t, h) in queries {
    match t {
      1 => {
        heap.push(Reverse(h));
      },
      _ =>
        while let Some(&Reverse(min_val)) = heap.peek() {
          if min_val <= h {
            heap.pop();
          } else {
            break;
          }
        },
    }
    println!("{}", heap.len());
  }
}

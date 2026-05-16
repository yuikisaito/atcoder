use std::cmp::Reverse;
use std::collections::BinaryHeap;

use proconio::input;

fn push(
  l: &mut BinaryHeap<usize>,
  s: &mut BinaryHeap<Reverse<usize>>,
  val: usize,
) {
  if val <= *l.peek().unwrap() {
    l.push(val);
  } else {
    s.push(Reverse(val));
  }

  if l.len() <= s.len() {
    let Reverse(top) = s.pop().unwrap();
    l.push(top);
  }
  if l.len() > s.len() + 1 {
    let top = l.pop().unwrap();
    s.push(Reverse(top));
  }
}

fn main() {
  input! {
    x: usize,
    q: usize,
    ab: [(usize, usize); q],
  }

  let mut l = BinaryHeap::from([x]); // i + 1
  let mut s = BinaryHeap::new(); // i

  for (a, b) in ab {
    push(&mut l, &mut s, a);
    push(&mut l, &mut s, b);

    println!("{}", l.peek().unwrap());
  }
}

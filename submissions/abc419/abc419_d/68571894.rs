use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;

fn main() {
  input! {
    n: usize,
    m: usize,
    s: Chars,
    t: Chars,
    lr: [(usize, usize); m],
  }

  let mut order = [s, t].concat();

  for &(l, r) in lr.iter() {
    for i in l - 1..r {
      order.swap(i, i + n);
    }
  }

  println!("{}", order[0..n].iter().join(""));
}

use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    c: [usize; n],
  }

  let hm = c.into_iter().counts();

  println!("{}", n - hm.values().max().unwrap());
}

use std::cmp::Ordering;
use std::cmp::{
  self,
};

use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  println!("{}", a.into_iter().tuple_windows::<(_, _)>().map(|(x, y)| y.cmp(&x)).tuple_windows::<(_, _)>().filter(|x| *x == (Ordering::Greater, Ordering::Less)).count());
}

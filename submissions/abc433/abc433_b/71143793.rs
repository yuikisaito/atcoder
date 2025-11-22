use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    a: [usize; n],
  }

  for i in 0..n {
    let filtered = a[0..i].iter().enumerate().filter(|&(_, &ai)| ai > a[i]);
    if filtered.clone().count() == 0 {
      println!("-1");
    } else {
      println!("{}", filtered.sorted().rev().next().unwrap().0 + 1);
    }
  }
}

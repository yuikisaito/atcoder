use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    v: usize,
    w: [usize; n],
  }

  println!("{}", (1..=n).combinations(3).filter(|x| x.iter().sum::<usize>() <= v).map(|x| x.into_iter().map(|i| w[i - 1]).sum::<usize>()).max().unwrap());
}

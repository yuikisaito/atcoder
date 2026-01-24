use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    ab: [(usize, usize); m],
  }

  let mut authors = vec![0; n];
  for (a, b) in ab {
    authors[a - 1] += 1;
    authors[b - 1] += 1;
  }

  for cons in authors {
    let kouho = n - 1 - cons;
    println!("{}", (0..kouho).combinations(3).count())
  }
}

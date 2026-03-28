use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    ab: [(usize, usize); n],
  }

  let mut now = vec![0; m];
  let mut next = vec![0; m];
  for (a, b) in ab {
    now[a - 1] += 1;
    next[b - 1] += 1;
  }

  println!("{}", now.into_iter().zip(next.into_iter()).map(|(x, y)| y - x).join("\n"));
}

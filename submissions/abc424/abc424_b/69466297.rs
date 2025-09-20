use itertools::Itertools;
use proconio::input;

fn main() {
  input! {
    n: usize,
    m: usize,
    k: usize,
    ab: [(usize, usize); k],
  }

  let mut member = vec![0; n];
  let mut seikai = Vec::new();

  for (a, _) in ab {
    member[a - 1] += 1;
    if member[a - 1] == m {
      seikai.push(a);
    }
  }

  println!("{}", seikai.iter().join(" "))
}

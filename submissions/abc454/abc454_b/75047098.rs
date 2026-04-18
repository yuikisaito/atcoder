use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
  input! {
    n: usize,
    m: usize,
    f: [Usize1; n],
  }

  println!("{}", if f.iter().sorted().dedup().count() == f.len() { "Yes" } else { "No" });

  let mut clothes = vec![false; m];
  f.iter().for_each(|&i| clothes[i] = true);
  println!("{}", if clothes.into_iter().all(|i| i) { "Yes" } else { "No" });
}
